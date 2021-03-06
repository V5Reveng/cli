//! The public interface to the Device.

use super::send as priv_send;
use super::CommandId;
use crate::device::{filesystem, helpers, receive, send};
use crate::device::{Device, DeviceError, ProtocolError, ResponseByte, Result};
use log::{debug, warn};

impl Device {
	pub fn device_info(&mut self) -> Result<receive::DeviceInfo> {
		debug!("sending device info command");
		self.simple_command_no_data(0xa4)
	}

	fn has_new_ext_dev_info(&mut self) -> Result<bool> {
		let device_info = self.device_info()?;
		Ok(match device_info.product {
			helpers::Product::Brain(_) => device_info.version >= helpers::LongVersion::new(1, 0, 13, 0, 0),
			helpers::Product::Controller(_) => false,
		})
	}
	pub fn extended_device_info(&mut self) -> Result<receive::ExtendedDeviceInfo> {
		debug!("sending extended device info command");
		const COMMAND_ID: CommandId = 0x22;
		if self.has_new_ext_dev_info()? {
			self.ext_command_no_data::<receive::ExtendedDeviceInfoNew>(COMMAND_ID).map(receive::ExtendedDeviceInfo::from)
		} else {
			self.ext_command_no_data::<receive::ExtendedDeviceInfo>(COMMAND_ID)
		}
	}

	/// `Ok(None)` is returned if the file does not exist.
	pub fn get_file_metadata_by_name(&mut self, args: &send::FileMetadataByName) -> Result<Option<receive::FileMetadataByName>> {
		debug!("sending get-file-metadata-by-name command");
		let ret = self.ext_command_with_data::<_, receive::FileMetadataByName>(0x19, args);
		match ret {
			Ok(data) => Ok(Some(data)),
			Err(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::Enoent | ResponseByte::ProgramFileError))) => Ok(None),
			Err(err) => Err(err),
		}
	}
	/// `Ok(None)` is returned if the file does not exist.
	///
	/// This is mostly used to either get the name of a file by its index, or to list the contents of a category.
	/// The latter can also be done with `list_all_files`.
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
		let (size, address) = match (args.size, args.address) {
			(Some(size), Some(address)) => (size, address),
			(maybe_size, maybe_address) => {
				let file_metadata = self
					.get_file_metadata_by_name(&send::FileMetadataByName::new(&file.common))?
					.ok_or(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::Enoent)))?;
				let size = maybe_size.unwrap_or(file_metadata.size);
				let address = maybe_address.unwrap_or(file_metadata.address);
				(size, address)
			}
		};
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
		if !args.ignore_crc && crc != transfer_info.crc {
			Err(DeviceError::Protocol(ProtocolError::InvalidCrc))
		} else {
			Ok(())
		}
	}

	/// Write to the device from the specified stream. You will need to also provide the size of the file and the CRC beforehand. If you don't want to calculate them yourself, you can use `write_file_from_slice`.
	pub fn write_file_from_stream(&mut self, stream: &mut dyn std::io::Read, file: &filesystem::QualFile, size: filesystem::FileSize, crc: u32, args: &filesystem::WriteArgs) -> Result<()> {
		let address = match args.address {
			Some(addr) => addr,
			None => self.get_file_metadata_by_name(&send::FileMetadataByName::new(&file.common))?.map(|x| x.address).unwrap_or(filesystem::DEFAULT_ADDRESS),
		};
		self.set_transfer_channel(filesystem::Channel::FileTransfer)?;
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
		self.ft_write(stream, size, address, transfer_info.max_packet_size)?;
		// a large file transfer can take a while to end, so set the timeout as such. A 500 KB file would result in a timeout of 10 seconds.
		self.set_timeout(std::time::Duration::from_millis(std::cmp::max(size / 50, 1000) as u64))?;
		self.end_file_transfer(args.action)?;
		self.reset_timeout()?;
		self.set_transfer_channel(filesystem::Channel::Pit)?;
		Ok(())
	}
	/// Write to a file from a slice. The size will be the size of the slice, and the CRC will be calculated for you.
	pub fn write_file_from_slice(&mut self, data: &[u8], file: &filesystem::QualFile, args: &filesystem::WriteArgs) -> Result<()> {
		let mut stream = encde::util::SliceReader::new(data);
		let crc = *<u32 as crate::crc::CrcComputable>::update_crc(&mut 0u32, data);
		let size: filesystem::FileSize = data.len().try_into().expect("Data to be written is too large");
		self.write_file_from_stream(&mut stream, file, size, crc, args)
	}

	pub fn delete_file(&mut self, file: &filesystem::QualFileName, args: &filesystem::DeleteArgs) -> Result<bool> {
		let ret = self.ext_command_with_data::<_, ()>(0x1b, &priv_send::DeleteFile::new(file, args.include_linked));
		let was_deleted = match ret {
			Ok(_) => true,
			Err(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::Enoent | ResponseByte::ProgramFileError))) => false,
			Err(err) => {
				return Err(err);
			}
		};
		// I'm not convinced that this is necessary, but the PROS CLI includes it.
		if was_deleted {
			self.end_file_transfer(Default::default())?;
		}
		Ok(was_deleted)
	}

	pub fn capture_screen(&mut self, output_stream: &mut dyn std::io::Write) -> Result<()> {
		self.prepare_screen_capture()?;
		self.receive_screen_capture(output_stream)
	}

	pub fn execute_file(&mut self, file: &filesystem::QualFileName) -> Result<()> {
		self.ext_command_with_data::<_, ()>(0x18, &priv_send::ExecuteFile::start(file))
	}

	pub fn stop_execution(&mut self) -> Result<()> {
		self.ext_command_with_data::<_, ()>(0x18, &priv_send::ExecuteFile::stop())
	}
}
