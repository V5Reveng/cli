use crate::commands::Runnable;
use crate::device;

#[derive(clap::Parser)]
/// Print device info.
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		let devices = device::UploadableInfo::get_all().unwrap();
		for device in devices.iter() {
			println!("Device {} of type {}", device.name, device.device_type);
		}
		0
	}
}
