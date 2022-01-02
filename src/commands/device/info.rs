use crate::commands::Runnable;

#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(&mut self, dev: crate::presence::Presence<crate::device::Device>) {
		let mut dev = dev.expect_one();
		let dev_info = dev.device_info().unwrap();
		let ext_dev_info = dev.extended_device_info().unwrap();
		println!("Device type: {}", dev_info.product);
		println!("System version: {}", dev_info.version);
		println!("CPU versions: {} {}", ext_dev_info.cpu0_version, ext_dev_info.cpu1_version);
		println!("Touch version: {}", ext_dev_info.touch_version);
		println!("System ID: {:08x}", ext_dev_info.system_id);
	}
}
