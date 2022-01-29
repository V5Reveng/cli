//! Errors that can occur during communication with the Device.

use super::ResponseByte;

/// The variants exactly match the `serialport::ErrorKind` variants, excluding `Io`, which is promoted to `DeviceError::Io`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SerialError {
	InvalidInput,
	Unknown,
	NoDevice,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolError {
	WrongData { entity: &'static str, expected: Box<[u8]>, received: Box<[u8]> },
	BadLength { entity: &'static str, received_length: usize },
	OutOfRange { entity: &'static str, min: usize, max: usize, actual: usize },
	Nack(ResponseByte),
	InvalidCrc,
}

#[derive(Debug)]
pub enum DeviceError {
	Io(std::io::Error),
	Serial(SerialError),
	Encde(encde::Error),
	Protocol(ProtocolError),
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
