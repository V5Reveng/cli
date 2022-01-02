use super::{BRAIN_PRODUCT_ID, CONTROLLER_PRODUCT_ID, VEX_VENDOR_ID};
use log::warn;
use serialport::{available_ports, SerialPortType};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct UsbPort {
	name: String,
	info: serialport::UsbPortInfo,
}

fn get_usb_devices() -> serialport::Result<Vec<UsbPort>> {
	let raw = available_ports()?;
	Ok(raw
		.into_iter()
		.filter_map(|port| match port.port_type {
			SerialPortType::UsbPort(info) => Some(UsbPort { name: port.port_name, info }),
			_ => None,
		})
		.collect())
}

#[derive(PartialEq, Eq)]
pub enum Classification {
	NotVex,
	/// Has the Vex vendor ID, but an unknown product ID
	UnknownVexDevice,
	/// The system processor's serial port
	Brain,
	/// The user processor's serial port
	BrainUser,
	Controller,
}

#[cfg(target_os = "linux")]
fn get_device_location(dev_name: &str) -> Result<u8, String> {
	// given a device /dev/tty(subsystem)(index)
	let dev_name = Path::new(dev_name);
	if !dev_name.starts_with("/dev") {
		warn!("Device {} is not in /dev but the filename does have the correct format; results may be inaccurate", dev_name.display());
	}
	// get the filename
	let dev_name = dev_name.file_name().ok_or("Invalid device path")?.to_str().ok_or("Non-UTF8 device path")?;
	// this part is dependent on the specific subsystem
	if dev_name.starts_with("ttyACM") {
		// given a device /dev/ttyACM0 (dev_name = "ttyACM0"), there will be a sysfs entry under /sys/class/tty/ttyACM0
		let sys_path: PathBuf = ["/sys/class/tty", dev_name].iter().collect();
		// this should be a link to /sys/devices/pci????:??/????:??:*.*/usb*/*-*/*-*:*.(location)/tty/ttyACM0
		let sys_device = fs::read_link(sys_path).map_err(|e| format!("{}", e))?;
		// get the component with the location as a string
		let usb_bus_info = sys_device
			.parent()
			.ok_or("Invalid /sys/devices path")?
			.parent()
			.ok_or("Invalid /sys/devices path")?
			.file_name()
			.ok_or("Invalid /sys/devices path")?
			.to_str()
			.ok_or("Non-UTF8 /sys/devices path")?;
		// get the part after the dot (i.e., the location)
		let location = usb_bus_info.rsplit_once('.').ok_or("Invalid USB underlying device path format")?.1;
		location.parse().map_err(|e| format!("{}", e))
	} else {
		todo!("Implement device location detection for subsystem of {}", dev_name);
	}
}

fn classify(port: &UsbPort) -> Classification {
	match port.info.vid {
		VEX_VENDOR_ID => match port.info.pid {
			CONTROLLER_PRODUCT_ID => Classification::Controller,
			BRAIN_PRODUCT_ID => match get_device_location(&port.name).expect("Device location detection failed") {
				0 => Classification::Brain,
				2 => Classification::BrainUser,
				_ => Classification::UnknownVexDevice,
			},
			_ => Classification::UnknownVexDevice,
		},
		_ => Classification::NotVex,
	}
}

#[derive(Debug, Clone, Copy)]
pub enum UploadableType {
	Brain,
	Controller,
}
impl std::fmt::Display for UploadableType {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		fmt.write_str(match self {
			UploadableType::Brain => "brain",
			UploadableType::Controller => "controller",
		})
	}
}

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

#[derive(Debug)]
pub enum UploadableInfoFromPathError {
	/// Path is not valid UTF-8
	PathNotUTF8,
	/// Path does not exist
	Nonexistent,
	/// Error in the underlying serialport library
	SerialPortError(serialport::Error),
	/// The path, in some way, does not refer to a valid uploadable device. Among others:
	/// - The path does exist, but is not a serial port.
	/// - The path is a serial port, but not an uploadable VEX device as identified by the platform-specific implementation.
	NotValid,
}

impl From<serialport::Error> for UploadableInfoFromPathError {
	fn from(e: serialport::Error) -> UploadableInfoFromPathError {
		UploadableInfoFromPathError::SerialPortError(e)
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
