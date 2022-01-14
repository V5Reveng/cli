use super::location::get_device_location;
use super::usb_port::UsbPort;

const VEX_VENDOR_ID: u16 = 0x2888;
const CONTROLLER_PRODUCT_ID: u16 = 0x0503;
const BRAIN_PRODUCT_ID: u16 = 0x0501;

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

pub fn classify(port: &UsbPort) -> Classification {
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
