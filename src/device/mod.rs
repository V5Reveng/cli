pub mod discover;
pub mod error;
pub mod filesystem;
mod helpers;
mod r#impl;
pub mod receive;
pub mod response_byte;
pub mod send;

pub use discover::{UploadableInfo, UploadableType};
pub use error::*;
pub use response_byte::ResponseByte;

pub struct Device {
	ty: UploadableType,
	port: crate::crc::CRCSerialPort,
}
impl std::fmt::Debug for Device {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(formatter, "Device of type {} at {}", self.ty, self.port.port().name().unwrap_or_else(|| "(unknown)".to_owned()))
	}
}
