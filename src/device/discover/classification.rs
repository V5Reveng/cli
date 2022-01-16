//! Classification of USB ports.

use super::location::get_device_location;
use super::usb_port::UsbPort;

const VEX_VENDOR_ID: u16 = 0x2888;
const CONTROLLER_PRODUCT_ID: u16 = 0x0503;
const BRAIN_PRODUCT_ID: u16 = 0x0501;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

pub fn classify(port: &UsbPort) -> Classification {
	use Classification::*;
	match port.info.vid {
		VEX_VENDOR_ID => match port.info.pid {
			CONTROLLER_PRODUCT_ID => Controller,
			BRAIN_PRODUCT_ID => match get_device_location(&port.name).expect("Device location detection failed") {
				0 => Brain,
				2 => BrainUser,
				_ => UnknownVexDevice,
			},
			_ => UnknownVexDevice,
		},
		_ => NotVex,
	}
}
