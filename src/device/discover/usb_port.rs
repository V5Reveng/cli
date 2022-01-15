use serialport::{available_ports, Result, SerialPortType, UsbPortInfo};

#[derive(Debug)]
pub struct UsbPort {
	pub name: String,
	/// Pretty restricted. `UploadableInfo` can give more information.
	pub info: UsbPortInfo,
}

pub fn get_usb_devices() -> Result<Vec<UsbPort>> {
	let raw = available_ports()?;
	Ok(raw
		.into_iter()
		.filter_map(|port| match port.port_type {
			SerialPortType::UsbPort(info) => Some(UsbPort { name: port.port_name, info }),
			_ => None,
		})
		.collect())
}
