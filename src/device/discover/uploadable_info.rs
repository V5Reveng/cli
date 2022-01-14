use super::classification::{classify, Classification};
use super::usb_port::{get_usb_devices, UsbPort};
use super::{UploadableInfoFromPathError, UploadableType};
use std::path::Path;

pub struct UploadableInfo {
	pub name: String,
	pub device_type: UploadableType,
}

impl TryFrom<UsbPort> for UploadableInfo {
	/// This is a bit ugly, but we can just pass the (non-uploadable) device type as the error
	type Error = Classification;
	fn try_from(port: UsbPort) -> Result<Self, Self::Error> {
		match classify(&port) {
			Classification::Brain => Ok(Self {
				name: port.name,
				device_type: UploadableType::Brain,
			}),
			Classification::Controller => Ok(Self {
				name: port.name,
				device_type: UploadableType::Controller,
			}),
			other => Err(other),
		}
	}
}

impl TryFrom<&Path> for UploadableInfo {
	type Error = UploadableInfoFromPathError;
	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		//! FIXME this might be able to be improved
		if !path.exists() {
			return Err(UploadableInfoFromPathError::Nonexistent);
		}
		let path = path.to_str().ok_or(UploadableInfoFromPathError::PathNotUTF8)?;
		Self::get_all().map_err(UploadableInfoFromPathError::from)?.into_iter().find(|port| port.name == path).ok_or(UploadableInfoFromPathError::NotValid)
	}
}

impl UploadableInfo {
	pub fn get_all() -> serialport::Result<Vec<UploadableInfo>> {
		Ok(get_usb_devices()?.into_iter().filter_map(|x| UploadableInfo::try_from(x).ok()).collect::<Vec<_>>())
	}
}
