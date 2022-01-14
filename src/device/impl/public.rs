use super::send as priv_send;
use super::CommandId;
use crate::device::{filesystem, helpers, receive, send};
use crate::device::{Device, DeviceError, ProtocolError, ResponseByte, Result};
use log::{debug, warn};

impl Device {
	/// Get the basic device information: version and product.
	pub fn device_info(&mut self) -> Result<receive::DeviceInfo> {
		debug!("sending device info command");
		self.simple_command_no_data(0xa4)
	}

	fn has_new_ext_dev_info(&mut self) -> Result<bool> {
		let device_info = self.device_info()?;
		let min_version = match device_info.product {
			helpers::Product::Brain(_) => helpers::LongVersion::new(1, 0, 13, 0, 0),
			helpers::Product::Controller(_) => helpers::LongVersion::new(1, 0, 0, 0, 70),
		};
		Ok(device_info.version >= min_version)
	}
	/// Get the extended device information: system and CPU versions, touch version, and system ID.
	pub fn extended_device_info(&mut self) -> Result<receive::ExtendedDeviceInfo> {
		debug!("sending extended device info command");
		const COMMAND_ID: CommandId = 0x22;
		if self.has_new_ext_dev_info()? {
			self.ext_command_no_data::<receive::ExtendedDeviceInfoNew>(COMMAND_ID).map(receive::ExtendedDeviceInfo::from)
		} else {
			self.ext_command_no_data::<receive::ExtendedDeviceInfo>(COMMAND_ID)
		}
	}

	pub fn get_file_metadata_by_name(&mut self, args: &send::FileMetadataByName) -> Result<Option<receive::FileMetadataByName>> {
		debug!("sending get-file-metadata-by-name command");
		let ret = self.ext_command_with_data::<_, receive::FileMetadataByName>(0x19, args);
		match ret {
			Ok(data) => Ok(Some(data)),
			Err(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::Enoent | ResponseByte::ProgramFileError))) => Ok(None),
			Err(err) => Err(err),
		}
	}
	pub fn get_file_metadata_by_index(&mut self, index: filesystem::FileIndex) -> Result<Option<receive::FileMetadataByIndex>> {
		let ret = self.ext_command_with_data::<_, receive::FileMetadataByIndex>(0x17, &send::FileMetadataByIndex::new(index));
		match ret {
			Ok(data) => Ok(Some(data)),
			Err(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::Enoent | ResponseByte::ProgramFileError))) => Ok(None),
			Err(err) => Err(err),
		}
	}

	pub fn num_files(&mut self, category: filesystem::Category) -> Result<isize> {
		debug!("sending num-files command");
		self.ext_command_with_data::<_, receive::NumFiles>(0x16, &send::NumFiles::new(category)).map(|receive::NumFiles(num)| num as isize)
	}
	pub fn list_all_files(&mut self, category: filesystem::Category) -> Result<Vec<receive::FileMetadataByIndex>> {
		debug!("listing all files");
		let num_files: usize = self.num_files(category)?.try_into().expect("The number of files was negative");
		let num_files = if num_files > (u8::MAX as usize) {
			warn!("There are too many files to list all of them; only listing the first {}", u8::MAX);
			u8::MAX
		} else {
			num_files as u8
		};
		let mut ret = Vec::with_capacity(num_files as usize);
		for i in 0..num_files {
			ret.push(self.get_file_metadata_by_index(i)?.ok_or(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::Enoent)))?)
		}
		Ok(ret)
	}

	pub fn read_file_to_stream(&mut self, stream: &mut dyn std::io::Write, file: &filesystem::QualFile, args: &filesystem::ReadArgs) -> Result<()> {
		debug!("reading file {}", file);
		let file_metadata = self
			.get_file_metadata_by_name(&send::FileMetadataByName::new(&file.common))?
			.ok_or(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::Enoent)))?;
		let size = args.size.unwrap_or(file_metadata.size);
		let address = args.address.unwrap_or(file_metadata.address);
		let transfer_info = self.start_file_transfer(&priv_send::StartFileTransfer {
			function: filesystem::Function::Download,
			target: args.target,
			category: file.common.category,
			overwrite: false,
			size,
			address,
			crc: 0,
			file_type: file.ty,
			timestamp: Default::default(),
			version: helpers::ShortVersion::new(1, 0, 0, 0),
			name: file.common.name,
		})?;
		let crc = self.ft_read(stream, size, address, transfer_info.max_packet_size)?;
		self.end_file_transfer(filesystem::TransferCompleteAction::default())?;
		if crc != transfer_info.crc {
			Err(DeviceError::Protocol(ProtocolError::InvalidCrc))
		} else {
			Ok(())
		}
	}

	pub fn write_file_from_stream(&mut self, stream: &mut dyn std::io::Read, file: &filesystem::QualFile, size: filesystem::FileSize, crc: u32, args: &filesystem::WriteArgs) -> Result<()> {
		let address = match args.address {
			Some(addr) => addr,
			None => self.get_file_metadata_by_name(&send::FileMetadataByName::new(&file.common))?.map(|x| x.address).unwrap_or(0x0780_0000),
		};
		let transfer_info = self.start_file_transfer(&priv_send::StartFileTransfer {
			function: filesystem::Function::Upload,
			target: filesystem::Target::Flash,
			category: file.common.category,
			overwrite: args.overwrite,
			size,
			address,
			crc,
			file_type: file.ty,
			timestamp: args.timestamp,
			version: helpers::ShortVersion::new(1, 0, 0, 0),
			name: file.common.name,
		})?;
		if transfer_info.file_size < size {
			return Err(DeviceError::Protocol(ProtocolError::BadLength {
				entity: "echoed length of file to write",
				received_length: transfer_info.file_size as usize,
			}));
		}
		if let Some(ref linked_file) = args.linked_file {
			self.ft_set_link(linked_file)?;
		}
		// if this doesn't work, try halving the max_packet_size
		self.ft_write(stream, size, address, transfer_info.max_packet_size)?;
		self.end_file_transfer(args.action)?;
		Ok(())
	}
	pub fn write_file_from_slice(&mut self, data: &[u8], file: &filesystem::QualFile, args: &filesystem::WriteArgs) -> Result<()> {
		let mut stream = encde::util::SliceReader::new(data);
		let crc = *<u32 as crate::crc::CrcComputable>::update_crc(&mut 0u32, data);
		let size: filesystem::FileSize = data.len().try_into().expect("Data to be written is too large");
		self.write_file_from_stream(&mut stream, file, size, crc, args)
	}

	pub fn delete_file(&mut self, file: &filesystem::QualFileName, args: &filesystem::DeleteArgs) -> Result<()> {
		self.ext_command_with_data::<_, ()>(0x1b, &priv_send::DeleteFile::new(file, args.include_linked))?;
		// I'm not convinced that this is necessary, but the PROS CLI includes it.
		self.end_file_transfer(Default::default())?;
		Ok(())
	}
}
