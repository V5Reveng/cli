//! This module can be used to get the list of uploadable V5 devices. (Uploadable refers to devices to which a program can be uploaded.)

use super::classification::Classification;
use super::usb_port::UsbPort;
use super::{UploadableInfoFromPathError as FPError, UploadableType};
use std::path::Path;

pub struct UploadableInfo {
	/// On platforms that use paths to represent serial devices (Windows, Unix, more?), this is that path.
	pub name: String,
	pub device_type: UploadableType,
}

/// The device can possible be converted from a USB port, as long as the USB port has an uploadable device connected.
impl TryFrom<UsbPort> for UploadableInfo {
	/// This is a bit ugly, but we can just pass the (non-uploadable) device type as the error
	type Error = Classification;
	fn try_from(port: UsbPort) -> Result<Self, Self::Error> {
		match Classification::classify(&port) {
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

/// You can get UploadableInfo for a Path, but it's not very elegant or performant.
impl TryFrom<&Path> for UploadableInfo {
	type Error = FPError;
	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		//! FIXME this might be able to be improved
		if !path.exists() {
			return Err(FPError::Nonexistent);
		}
		let path = path.to_str().ok_or(FPError::PathNotUtf8)?;
		Self::get_all().map_err(FPError::from)?.into_iter().find(|port| port.name == path).ok_or(FPError::NotValid)
	}
}

impl UploadableInfo {
	pub fn get_all() -> serialport::Result<Vec<UploadableInfo>> {
		Ok(UsbPort::get_all()?.into_iter().filter_map(|x| UploadableInfo::try_from(x).ok()).collect::<Vec<_>>())
	}
}
