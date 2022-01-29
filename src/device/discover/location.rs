//! Get the location for a device.
//! The location is the last number in the position of a USB device, and is used to distinguish between the user and system port of the VEX V5.
//! Implementation is platform-dependent.

#[cfg(unix)]
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

#[cfg(windows)]
mod windows_impl {
	use std::ffi::c_void;
	use std::ptr::null_mut;
	use windows::Win32::Devices::Usb as winusb;
	use windows::Win32::Foundation as handleapi;
	use windows::Win32::Storage::FileSystem as fileapi;

	struct HandleWrapper(pub handleapi::HANDLE);
	impl Drop for HandleWrapper {
		fn drop(&mut self) {
			unsafe {
				handleapi::CloseHandle(self.0);
			}
		}
	}

	struct UsbInterfaceWrapper(pub *const c_void);
	impl Drop for UsbInterfaceWrapper {
		fn drop(&mut self) {
			unsafe {
				winusb::WinUsb_Free(self.0);
			}
		}
	}

	pub fn get_device_location(dev_name: &str) -> Result<u8, String> {
		println!("port is {}", dev_name);
		unsafe {
			let mut file_name = std::ffi::CString::new(format!(r"\\.\{}", dev_name)).unwrap().into_bytes_with_nul();
			let file = fileapi::CreateFileA(
				handleapi::PSTR(file_name.as_mut_ptr()),
				fileapi::FILE_GENERIC_READ | fileapi::FILE_GENERIC_WRITE,
				fileapi::FILE_SHARE_READ | fileapi::FILE_SHARE_WRITE,
				null_mut(),
				fileapi::OPEN_EXISTING,
				fileapi::FILE_FLAG_OVERLAPPED | fileapi::FILE_ATTRIBUTE_NORMAL,
				None,
			)
			.ok()
			.map_err(|err| err.message().to_string_lossy())?;
			let file = HandleWrapper(file);
			let mut handle = null_mut();
			// println!("file {}, handle {:x}, ptr {:x}", file.0 as usize, handle as usize, &mut handle as winusb::PWINUSB_INTERFACE_HANDLE as usize);
			winusb::WinUsb_Initialize(file.0, &mut handle).ok().map_err(|err| err.message().to_string_lossy())?;
			let handle = UsbInterfaceWrapper(handle);
			let mut buffer = [0u8; 0x80];
			let mut read_amount = 0u32;
			winusb::WinUsb_GetDescriptor(handle.0, winusb::USB_DEVICE_DESCRIPTOR_TYPE as u8, 0, 0, buffer.as_mut_ptr(), buffer.len() as u32, &mut read_amount)
				.ok()
				.map_err(|err| err.message().to_string_lossy())?;
			panic!("result for {}: {:?}", dev_name, &buffer[0..(read_amount as usize)]);
		}
	}
}

#[cfg(windows)]
pub use windows_impl::get_device_location;

#[cfg(not(any(unix, windows)))]
pub fn get_device_location(dev_name: &str) -> Result<u8, String> {
	compile_error!("get_device_location is not implemented for your system");
	unreachable!();
}
