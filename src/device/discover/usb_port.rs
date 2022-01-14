use serialport::{available_ports, SerialPortType};

#[derive(Debug)]
pub struct UsbPort {
	pub name: String,
	pub info: serialport::UsbPortInfo,
}

pub fn get_usb_devices() -> serialport::Result<Vec<UsbPort>> {
	let raw = available_ports()?;
	Ok(raw
		.into_iter()
		.filter_map(|port| match port.port_type {
			SerialPortType::UsbPort(info) => Some(UsbPort { name: port.port_name, info }),
			_ => None,
		})
		.collect())
}
