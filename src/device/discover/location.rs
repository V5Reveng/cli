#[cfg(target_os = "linux")]
pub fn get_device_location(dev_name: &str) -> Result<u8, String> {
	use log::warn;
	use std::fs;
	use std::path::{Path, PathBuf};

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
