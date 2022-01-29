//! Errors that can occur during communication with the Device.

use super::ResponseByte;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The variants exactly match the `serialport::ErrorKind` variants, excluding `Io`, which is promoted to `DeviceError::Io`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SerialError {
	InvalidInput,
	Unknown,
	NoDevice,
}

impl Error for SerialError {}
impl Display for SerialError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		let s = match self {
			Self::InvalidInput => "Invalid input",
			Self::Unknown => "Unknown error occurred",
			Self::NoDevice => "Device not available for communication",
		};
		formatter.write_str(s)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolError {
	WrongData { entity: &'static str, expected: Box<[u8]>, received: Box<[u8]> },
	BadLength { entity: &'static str, received_length: usize },
	OutOfRange { entity: &'static str, min: usize, max: usize, actual: usize },
	Nack(ResponseByte),
	InvalidCrc,
}

impl Error for ProtocolError {}
impl Display for ProtocolError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self {
			Self::WrongData { entity, expected, received } => write!(formatter, "Wrong data was received for {}: expected {:?} but got {:?}", entity, expected, received),
			Self::BadLength { entity, received_length } => write!(formatter, "The data received for {} was the wrong length ({})", entity, received_length),
			Self::OutOfRange { entity, min, max, actual } => write!(formatter, "The value received for {}, {}, was out of the range {} to {}", entity, actual, min, max),
			Self::Nack(response_byte) => write!(formatter, "The device sent a negative response (NACK): {}", response_byte),
			Self::InvalidCrc => write!(formatter, "The received CRC did not match the data"),
		}
	}
}

#[derive(Debug)]
pub enum DeviceError {
	Io(std::io::Error),
	Serial(SerialError),
	Encde(encde::Error),
	Protocol(ProtocolError),
	Other(Box<dyn Error + Send + Sync + 'static>),
}

impl DeviceError {
	pub fn category(&self) -> &'static str {
		match self {
			Self::Io(_) => "IO",
			Self::Serial(_) => "Serial",
			Self::Encde(_) => "Encoding/Decoding",
			Self::Protocol(_) => "Protocol",
			Self::Other(_) => "Other",
		}
	}
}
impl Error for DeviceError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(match self {
			Self::Io(e) => e,
			Self::Serial(e) => e,
			Self::Encde(e) => e,
			Self::Protocol(e) => e,
			Self::Other(e) => &**e,
		})
	}
}
impl Display for DeviceError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self.source() {
			Some(source) => write!(formatter, "{} error: {}", self.category(), source),
			None => write!(formatter, "{} error", self.category()),
		}
	}
}

impl From<std::io::Error> for DeviceError {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl From<serialport::Error> for DeviceError {
	fn from(err: serialport::Error) -> Self {
		use serialport::ErrorKind::*;
		match err.kind {
			Io(kind) => Self::Io(kind.into()),
			InvalidInput => Self::Serial(SerialError::InvalidInput),
			NoDevice => Self::Serial(SerialError::NoDevice),
			Unknown => Self::Serial(SerialError::Unknown),
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
