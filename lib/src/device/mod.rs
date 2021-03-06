use std::fmt::{self, Debug, Formatter};

pub mod discover;
pub mod error;
pub mod filesystem;
mod helpers;
// Maybe you're looking for this? All the actual code is in here.
mod r#impl;
pub mod receive;
pub mod response_byte;
pub mod send;

pub use discover::{UploadableInfo, UploadableType};
pub use error::*;
pub use response_byte::ResponseByte;

pub struct Device {
	ty: UploadableType,
	/// The serial port used to communicate with the device.
	port: crate::crc::CrcSerialPort,
}

impl Debug for Device {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(formatter, "Device of type {} at {}", self.ty, self.port.port().name().as_deref().unwrap_or("(unknown)"))
	}
}
