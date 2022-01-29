use crate::commands::Runnable;

/// List connected devices.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		let dev_info = dev.device_info()?;
		let ext_dev_info = dev.extended_device_info()?;
		println!("Device type: {}", dev_info.product);
		println!("System version: {}", dev_info.version);
		println!("CPU versions: {} {}", ext_dev_info.cpu0_version, ext_dev_info.cpu1_version);
		println!("Touch version: {}", ext_dev_info.touch_version);
		println!("System ID: {:08x}", ext_dev_info.system_id);
		Ok(())
	}
}
