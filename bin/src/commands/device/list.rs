use crate::commands::Runnable;
use v5_device::device;

/// Print device info.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: v5_device::util::presence::Presence<device::Device>) -> u32 {
		let devices = device::UploadableInfo::get_all().unwrap();
		for device in devices.iter() {
			println!("Device {} of type {}", device.name, device.device_type);
		}
		0
	}
}
