use crate::crc::CRCSerialPort;
use core::any::type_name;
use encde::{Decode, Encode};
use log::{debug, trace, warn};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

pub mod discover;
pub mod filesystem;
mod helpers;
pub mod receive;
pub mod send;
pub use discover::{UploadableInfo, UploadableType};

const SERIAL_BAUD: u32 = 115200;

pub struct Device {
	ty: UploadableType,
	port: CRCSerialPort,
}
impl std::fmt::Debug for Device {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(formatter, "Device of type {} at {}", self.ty, self.port.port().name().unwrap_or_else(|| "(unknown)".to_owned()))
	}
}
impl<'a> TryFrom<&'a Path> for Device {
	type Error = <discover::UploadableInfo as TryFrom<&'a Path>>::Error;
	fn try_from(path: &'a Path) -> std::result::Result<Self, Self::Error> {
		discover::UploadableInfo::try_from(path).map(Device::from)
	}
}
impl From<UploadableInfo> for Device {
	fn from(info: UploadableInfo) -> Self {
		use serialport::*;
		debug!("Opening serial port {} for V5 device of type {:?}", &info.name, &info.device_type);
		Device {
			ty: info.device_type,
			port: serialport::new(info.name, SERIAL_BAUD)
				.parity(Parity::None)
				.stop_bits(StopBits::One)
				.data_bits(DataBits::Eight)
				.flow_control(FlowControl::None)
				.timeout(Duration::from_secs(1))
				.open()
				.unwrap()
				.into(),
		}
	}
}

type CommandId = u8;

#[derive(Debug)]
pub enum SerialError {
	InvalidInput,
	Unknown,
	NoDevice,
}

#[derive(Debug)]
pub enum ProtocolError {
	WrongData { entity: &'static str, expected: Box<[u8]>, received: Box<[u8]> },
	BadLength { entity: &'static str, received_length: usize },
	OutOfRange { entity: &'static str, min: usize, max: usize, actual: usize },
	Nack(ResponseByte),
	InvalidCrc,
}

#[derive(Debug)]
pub enum DeviceError {
	IO(std::io::Error),
	Serial(SerialError),
	Encde(encde::Error),
	Protocol(ProtocolError),
}

impl From<std::io::Error> for DeviceError {
	fn from(err: std::io::Error) -> Self {
		Self::IO(err)
	}
}
impl From<serialport::Error> for DeviceError {
	fn from(err: serialport::Error) -> Self {
		match err.kind {
			serialport::ErrorKind::Io(kind) => Self::IO(kind.into()),
			serialport::ErrorKind::InvalidInput => Self::Serial(SerialError::InvalidInput),
			serialport::ErrorKind::NoDevice => Self::Serial(SerialError::NoDevice),
			serialport::ErrorKind::Unknown => Self::Serial(SerialError::Unknown),
		}
	}
}
impl From<encde::Error> for DeviceError {
	fn from(err: encde::Error) -> Self {
		Self::Encde(err)
	}
}
impl From<ProtocolError> for DeviceError {
	fn from(err: ProtocolError) -> Self {
		Self::Protocol(err)
	}
}

pub type Result<T> = std::result::Result<T, DeviceError>;

#[repr(u8)]
#[must_use = "This may be a NACK, which should be handled"]
#[derive(Encode, Decode, PartialEq, Eq, Debug)]
#[deny(missing_docs)]
pub enum ResponseByte {
	/// No error occurred.
	Ack = 0x76,
	/// An unspecified error occurred.
	GeneralNack = 0xff,
	/// Our CRC was invalid.
	ReceivedCrcError = 0xce,
	/// The payload is too small.
	// FIXME: needs more info
	PayloadTooSmall = 0xd0,
	/// The requested data would be too large to transfer.
	RequestedTransferTooLarge = 0xd1,
	/// The program CRC was invalid.
	// FIXME: this is a guess
	ProgramCrcError = 0xd2,
	/// An error occurred relating to program files.
	// FIXME: needs more info
	ProgramFileError = 0xd3,
	/// There was an attempt to upload or download uninitialized data.
	// FIXME: this is a guess
	UninitializedUploadDownload = 0xd4,
	/// The initialization was invalid for the file transfer type.
	// FIXME: this is a guess
	InitInvalidForFunction = 0xd5,
	/// The data is not aligned to 4 bytes.
	DataNotAligned = 0xd6,
	/// The file transfer packet address does not match the expected.
	PacketAddressWrong = 0xd7,
	/// Upon completion of the file transfer, the amount of received data did not match the length specified at the start of the file transfer.
	DownloadedLengthWrong = 0xd8,
	/// The requested file does not exist.
	Enoent = 0xd9,
	/// There is no space left on the device.
	Enospc = 0xda,
	/// The file already exists and the overwrite option was not specified.
	Eexist = 0xdb,
}

impl std::fmt::Display for ResponseByte {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::Ack => "Ack",
			Self::GeneralNack => "General Nack",
			Self::ReceivedCrcError => "Received CRC error",
			Self::PayloadTooSmall => "Payload too small",
			Self::RequestedTransferTooLarge => "Requested transfer too large",
			Self::ProgramCrcError => "Program CRC error",
			Self::ProgramFileError => "Program file error",
			Self::UninitializedUploadDownload => "Uninitialized upload download",
			Self::InitInvalidForFunction => "Initialization invalid for function",
			Self::DataNotAligned => "Data not aligned",
			Self::PacketAddressWrong => "Packet address wrong",
			Self::DownloadedLengthWrong => "Downloaded length wrong",
			Self::Enoent => "No such file or directory",
			Self::Enospc => "No space left on device",
			Self::Eexist => "File exists",
		})
	}
}

impl Device {
	fn rx<T: encde::Decode + std::fmt::Debug>(&mut self) -> Result<T> {
		let ret = encde::Decode::decode(&mut self.port)?;
		trace!("rx {}: {:?}", type_name::<T>(), ret);
		Ok(ret)
	}
	fn tx<T: encde::Encode + std::fmt::Debug>(&mut self, data: &T) -> Result<()> {
		trace!("tx {}: {:?}", type_name::<T>(), data);
		encde::Encode::encode(data, &mut self.port)?;
		Ok(())
	}
	fn rx_raw_data<const N: usize>(&mut self) -> Result<[u8; N]> {
		let mut output: [u8; N] = [0; N];
		self.port.read_exact(&mut output)?;
		trace!("rx {}: {:?}", type_name::<[u8; N]>(), output);
		Ok(output)
	}
	fn tx_raw_data(&mut self, data: &[u8]) -> Result<()> {
		trace!("tx {}: {:?}", type_name::<&[u8]>(), data);
		self.port.write_all(data)?;
		Ok(())
	}
}

impl Device {
	const EXT_COMMAND: CommandId = 0x56;

	fn tx_command_header(&mut self) -> Result<()> {
		debug!("tx command header");
		const HEADER: [u8; 4] = [0xc9, 0x36, 0xb8, 0x47];
		self.tx_raw_data(&HEADER)?;
		Ok(())
	}
	fn rx_response_header(&mut self) -> Result<()> {
		debug!("rx response header");
		let buf: [u8; 2] = self.rx_raw_data()?;
		const EXPECTED: [u8; 2] = [0xaa, 0x55];
		if buf != EXPECTED {
			Err(DeviceError::Protocol(ProtocolError::WrongData {
				entity: "Response header",
				expected: EXPECTED.into(),
				received: buf.into(),
			}))
		} else {
			Ok(())
		}
	}
	fn rx_echoed_command(&mut self, expected: CommandId) -> Result<()> {
		debug!("rx echoed command {:02x}", expected);
		let echoed_command: u8 = self.rx()?;
		if echoed_command != expected {
			Err(DeviceError::Protocol(ProtocolError::WrongData {
				entity: "Echoed command",
				expected: [expected].into(),
				received: [echoed_command].into(),
			}))
		} else {
			Ok(())
		}
	}
	fn rx_simple_payload_length(&mut self) -> Result<usize> {
		debug!("rx simple payload length");
		Ok(self.rx::<u8>()? as usize)
	}
	fn rx_bytes(&mut self, payload_length: usize) -> Result<Vec<u8>> {
		trace!("rx {} bytes", payload_length);
		let mut payload: Vec<u8> = vec![0; payload_length];
		self.port.read_exact(payload.as_mut_slice())?;
		Ok(payload)
	}
	fn rx_simple_raw_payload(&mut self) -> Result<Vec<u8>> {
		debug!("rx simple raw payload");
		let payload_length = self.rx_simple_payload_length()?;
		self.rx_bytes(payload_length)
	}
	fn decode_from_data<T: encde::Decode>(data: &[u8]) -> Result<T> {
		Ok(encde::util::decode_from_entire_slice(data)?)
	}
	fn rx_simple_payload<T: encde::Decode>(&mut self) -> Result<T> {
		debug!("rx simple payload");
		let raw = self.rx_simple_raw_payload()?;
		Self::decode_from_data(&raw)
	}
	fn tx_vex_varint(&mut self, length: usize) -> Result<()> {
		debug!("tx vex variable-length int: {}", length);
		match length {
			0..=0x7f => self.tx(&(length as u8)),
			0x80..=0x7fff => self.tx(&[((length >> 8) | 0x80) as u8, (length & 0xff) as u8]),
			actual => Err(DeviceError::Protocol(ProtocolError::OutOfRange {
				entity: "variable-length integer",
				min: 0,
				max: 0x7fff,
				actual,
			})),
		}
	}
	fn rx_vex_varint(&mut self) -> Result<usize> {
		let mut ret = self.rx::<u8>()? as usize;
		if ret & 0x80 == 0x80 {
			ret &= 0x7f;
			ret <<= 8;
			ret += self.rx::<u8>()? as usize;
		}
		debug!("rx vex variable-length int: {}", ret);
		Ok(ret)
	}
	fn rx_ext_payload_length(&mut self) -> Result<usize> {
		let ret = self.rx_vex_varint()?;
		debug!("rx extended payload length");
		Ok(ret)
	}
	fn rx_expect<T: encde::Decode + encde::Encode + PartialEq + std::fmt::Debug>(&mut self, entity: &'static str, expected: &T) -> Result<()> {
		let received = self.rx::<T>()?;
		if &received != expected {
			Err(DeviceError::Protocol(ProtocolError::WrongData {
				entity,
				expected: encde::util::encode_to_vec(expected)?.into(),
				received: encde::util::encode_to_vec(&received)?.into(),
			}))
		} else {
			trace!("rx {}, expecting value {:?}", entity, expected);
			Ok(())
		}
	}
	fn rx_response_byte(&mut self) -> Result<ResponseByte> {
		debug!("rx response byte");
		self.rx()
	}
}

impl Device {
	fn begin_simple_command(&mut self, command_id: CommandId) -> Result<()> {
		debug!("begin simple command {:#02x}", command_id);
		self.tx_command_header()?;
		self.tx(&command_id)?;
		Ok(())
	}
	fn end_simple_command<T: encde::Decode>(&mut self, command_id: CommandId) -> Result<T> {
		debug!("end simple command {:#02x}", command_id);
		self.rx_response_header()?;
		self.rx_echoed_command(command_id)?;
		self.rx_simple_payload()
	}
	fn tx_ext_command_header(&mut self, command_id: CommandId, payload_len: usize) -> Result<()> {
		debug!("begin extended command {:#02x} with {} bytes of data", command_id, payload_len);
		self.port.begin_tx_crc();
		self.tx_command_header()?;
		self.tx(&Self::EXT_COMMAND)?;
		self.tx(&command_id)?;
		self.tx_vex_varint(payload_len)?;
		Ok(())
	}
	fn tx_ext_command_footer(&mut self) -> Result<()> {
		self.port.end_tx_crc().map_err(DeviceError::from)
	}
	fn begin_ext_command(&mut self, command_id: CommandId, data: &[u8]) -> Result<()> {
		self.tx_ext_command_header(command_id, data.len())?;
		self.tx_raw_data(data)?;
		self.tx_ext_command_footer()?;
		Ok(())
	}
	fn rx_ext_command_header(&mut self, sent_command: CommandId) -> Result<usize> {
		trace!("rx extended command header");
		self.port.begin_rx_crc();
		self.rx_response_header()?;
		self.rx_expect("echoed command", &Self::EXT_COMMAND)?;
		// subtract echoed command byte, plus 16-bit CRC in rx_ext_command_footer
		let payload_len = self.rx_ext_payload_length()? - 3;
		self.rx_expect("echoed actual command", &sent_command)?;
		Ok(payload_len)
	}
	fn rx_ext_command_footer(&mut self) -> Result<()> {
		trace!("rx extended command footer");
		if self.port.end_rx_crc()? {
			Ok(())
		} else {
			Err(DeviceError::Protocol(ProtocolError::InvalidCrc))
		}
	}
	fn end_ext_command<T: encde::Decode>(&mut self, sent_command: CommandId) -> Result<T> {
		debug!("end extended command {:#02x}", sent_command);
		// subtract response byte
		let payload_len = self.rx_ext_command_header(sent_command)? - 1;
		let response_byte = self.rx_response_byte()?;
		let raw_payload = self.rx_bytes(payload_len)?;
		self.rx_ext_command_footer()?;
		if response_byte == ResponseByte::Ack {
			Self::decode_from_data(&raw_payload)
		} else {
			Err(ProtocolError::Nack(response_byte).into())
		}
	}
}

impl Device {
	fn simple_command_no_data<T: encde::Decode>(&mut self, command_id: CommandId) -> Result<T> {
		self.begin_simple_command(command_id)?;
		self.end_simple_command(command_id)
	}
	fn ext_command_no_data<T: encde::Decode>(&mut self, command_id: CommandId) -> Result<T> {
		self.begin_ext_command(command_id, &[])?;
		self.end_ext_command(command_id)
	}
	fn ext_command_with_data<S: encde::Encode, R: encde::Decode>(&mut self, command_id: CommandId, send_data: &S) -> Result<R> {
		let encoded = encde::util::encode_to_vec(send_data)?;
		self.begin_ext_command(command_id, &encoded)?;
		self.end_ext_command(command_id)
	}
}

/// Pad up to 4 byte boundary
fn pad(size: filesystem::PacketSize) -> filesystem::PacketSize {
	const BITS: filesystem::PacketSize = 4 - 1;
	let base = size & !BITS;
	let extra = size & BITS;
	let extra = if extra > 0 { 4 } else { 0 };
	base + extra
}

impl Device {
	fn start_file_transfer(&mut self, args: &send::StartFileTransfer) -> Result<receive::StartFileTransfer> {
		debug!("start file transfer");
		self.ext_command_with_data(0x11, &args)
	}
	fn ft_read_single(&mut self, data: &mut [u8], base_address: filesystem::Address) -> Result<()> {
		const COMMAND_ID: CommandId = 0x14;
		let amount_to_read: filesystem::PacketSize = data.len().try_into().expect("Buffer is too large to read with ft_read_single");
		let amount_to_read = pad(amount_to_read);
		debug!("file transfer: rx chunk of {} (padded to {}) bytes", data.len(), amount_to_read);
		let send = send::FileTransferRead { address: base_address, size: amount_to_read };
		self.begin_ext_command(COMMAND_ID, &encde::util::encode_to_vec(&send)?)?;
		let payload_len = self.rx_ext_command_header(COMMAND_ID)? - std::mem::size_of::<u32>();
		let _address = <u32 as encde::Decode>::decode(&mut self.port)?;
		if payload_len != amount_to_read as usize {
			return Err(DeviceError::Protocol(ProtocolError::BadLength {
				entity: "file transfer read packet",
				received_length: payload_len,
			}));
		}
		self.port.read_exact(data)?;
		encde::util::read_padding(&mut self.port, amount_to_read as usize - data.len())?;
		self.rx_ext_command_footer()?;
		Ok(())
	}
	/// Returns the CRC of the data that was read
	fn ft_read(&mut self, stream: &mut dyn std::io::Write, mut size: filesystem::FileSize, mut base_address: filesystem::Address, max_packet_size: filesystem::PacketSize) -> Result<u32> {
		debug!("file transfer: read {} bytes from 0x{:0>8x}, max packet size is {}", size, base_address, max_packet_size);
		let mut buffer = vec![0u8; max_packet_size as usize];
		let mut crc = 0;
		while size > 0 {
			let this_packet_size = std::cmp::min(size as usize, max_packet_size as usize);
			self.ft_read_single(&mut buffer[0..this_packet_size], base_address)?;
			<u32 as crate::crc::CrcComputable>::update_crc(&mut crc, &buffer[0..this_packet_size]);
			stream.write_all(&buffer[0..this_packet_size])?;
			base_address += filesystem::Address::try_from(this_packet_size).unwrap();
			size -= filesystem::FileSize::try_from(this_packet_size).unwrap();
		}
		Ok(crc)
	}
	fn ft_write_single(&mut self, data: &[u8], base_address: filesystem::Address) -> Result<()> {
		const COMMAND_ID: CommandId = 0x13;
		let amount_to_write: filesystem::PacketSize = data.len().try_into().expect("Buffer is too large to write with ft_write_single");
		let amount_to_write = pad(amount_to_write);
		debug!("file transfer: rx chunk of {} (padded to {}) bytes", data.len(), amount_to_write);
		self.tx_ext_command_header(COMMAND_ID, std::mem::size_of_val(&base_address) + amount_to_write as usize)?;
		base_address.encode(&mut self.port)?;
		self.port.write_all(data)?;
		encde::util::write_padding(&mut self.port, amount_to_write as usize - data.len())?;
		self.tx_ext_command_footer()?;
		self.end_ext_command::<()>(COMMAND_ID)?;
		Ok(())
	}
	fn ft_write(&mut self, stream: &mut dyn std::io::Read, mut size: filesystem::FileSize, mut base_address: filesystem::Address, max_packet_size: filesystem::PacketSize) -> Result<()> {
		debug!("file transfer: write {} to 0x{:0>8x}, max packet size is {}", size, base_address, max_packet_size);
		let mut buffer = vec![0u8; max_packet_size as usize];
		while size > 0 {
			let this_packet_size = std::cmp::min(size as usize, max_packet_size as usize);
			stream.read_exact(&mut buffer[0..this_packet_size])?;
			self.ft_write_single(&buffer[0..this_packet_size], base_address)?;
			base_address += filesystem::Address::try_from(this_packet_size).unwrap();
			size -= filesystem::FileSize::try_from(this_packet_size).unwrap();
		}
		Ok(())
	}
	fn ft_set_link(&mut self, linked_file: &filesystem::QualFileName) -> Result<()> {
		debug!("file transfer: set link to {}", linked_file);
		self.ext_command_with_data::<_, ()>(0x15, &send::FileTransferSetLink::new(linked_file))
	}
	fn end_file_transfer(&mut self, action: filesystem::TransferCompleteAction) -> Result<()> {
		debug!("end file transfer");
		self.ext_command_with_data::<_, ()>(0x12, &action)
	}
}

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
		let transfer_info = self.start_file_transfer(&send::StartFileTransfer {
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
		let transfer_info = self.start_file_transfer(&send::StartFileTransfer {
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
		self.ext_command_with_data::<_, ()>(0x1b, &send::DeleteFile::new(file, args.include_linked))?;
		// I'm not convinced that this is necessary, but the PROS CLI includes it.
		self.end_file_transfer(Default::default())?;
		Ok(())
	}
}
