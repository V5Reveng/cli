use crate::commands::Runnable;
use v5_device::device;

/// Print device info.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let devices = device::UploadableInfo::get_all()?;
		for device in devices.iter() {
			println!("Device {} of type {}", device.name, device.device_type);
		}
		Ok(())
	}
}
