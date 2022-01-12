use crate::commands::Runnable;
use crate::device;

#[derive(clap::Parser)]
pub struct Args {
	// no args
}

impl Runnable for Args {
	fn run(self, _dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		let devices = device::UploadableInfo::get_all().unwrap();
		for device in devices.iter() {
			println!("Device {} of type {}", device.name, device.device_type);
		}
		0
	}
}
