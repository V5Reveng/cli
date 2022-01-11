use crate::crc::CRCSerialPort;
use core::any::type_name;
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

const VEX_VENDOR_ID: u16 = 0x2888;
const CONTROLLER_PRODUCT_ID: u16 = 0x0503;
const BRAIN_PRODUCT_ID: u16 = 0x0501;

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
	InvalidEnumValue { entity: &'static str, value: usize },
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
#[derive(num_enum::IntoPrimitive, num_enum::TryFromPrimitive, PartialEq, Eq, Debug)]
pub enum ResponseByte {
	Ack = 0x76,
	GeneralNack = 0xff,
	ReceivedCrcError = 0xce,
	PayloadTooSmall = 0xd0,
	RequestedTransferTooLarge = 0xd1,
	ProgramCrcError = 0xd2,
	ProgramFileError = 0xd3,
	UninitializedUploadDownload = 0xd4,
	InitInvalidForFunction = 0xd5,
	DataNotAligned = 0xd6,
	PacketAddressWrong = 0xd7,
	DownloadedLengthWrong = 0xd8,
	Enoent = 0xd9,
	Enospc = 0xda,
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
		debug!("rx extended payload length");
		let ret = self.rx_vex_varint()?;
		match ret {
			4.. => Ok(ret - 4),
			received_length => Err(DeviceError::Protocol(ProtocolError::BadLength {
				entity: "extended payload length",
				received_length,
			})),
		}
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
	fn rx_response_byte(&mut self) -> Result<()> {
		debug!("rx response byte");
		let response_byte: u8 = self.rx()?;
		if let Ok(response_byte) = ResponseByte::try_from(response_byte) {
			if response_byte != ResponseByte::Ack {
				Err(DeviceError::Protocol(ProtocolError::Nack(response_byte)))
			} else {
				Ok(())
			}
		} else {
			Err(DeviceError::Protocol(ProtocolError::InvalidEnumValue {
				entity: "response byte",
				value: response_byte.into(),
			}))
		}
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
	fn begin_ext_command(&mut self, command_id: CommandId, data: &[u8]) -> Result<()> {
		debug!("begin extended command {:#02x} with {} bytes of data", command_id, data.len());
		self.port.begin_tx_crc();
		self.tx_command_header()?;
		self.tx(&Self::EXT_COMMAND)?;
		self.tx(&command_id)?;
		self.tx_vex_varint(data.len())?;
		self.tx_raw_data(data)?;
		self.port.end_tx_crc()?;
		Ok(())
	}
	fn end_ext_command<T: encde::Decode>(&mut self, sent_command: CommandId) -> Result<T> {
		debug!("end extended command {:#02x}", sent_command);
		self.port.begin_rx_crc();
		self.rx_response_header()?;
		self.rx_expect("echoed command", &Self::EXT_COMMAND)?;
		let payload_len = self.rx_ext_payload_length()?;
		self.rx_expect("echoed actual command", &sent_command)?;
		self.rx_response_byte()?;
		let raw_payload = self.rx_bytes(payload_len)?;
		if !self.port.end_rx_crc()? {
			Err(DeviceError::Protocol(ProtocolError::InvalidCrc))
		} else {
			Self::decode_from_data(&raw_payload)
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

impl Device {
	pub fn device_type(&self) -> UploadableType {
		self.ty
	}

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

	pub fn get_file_metadata_by_name(&mut self, args: &send::FileMetadataByName) -> Result<receive::FileMetadataByName> {
		debug!("sending get-file-metadata-by-name command");
		self.ext_command_with_data::<_, receive::FileMetadataByName>(0x19, args)
	}
	pub fn get_file_metadata_by_index(&mut self, index: filesystem::FileIndex) -> Result<receive::FileMetadataByIndex> {
		self.ext_command_with_data::<_, receive::FileMetadataByIndex>(0x17, &send::FileMetadataByIndex::new(index))
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
			ret.push(self.get_file_metadata_by_index(i)?)
		}
		Ok(ret)
	}
}
