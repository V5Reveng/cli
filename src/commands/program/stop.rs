use crate::commands::Runnable;

/// Stop the running program.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		dev.stop_execution().expect("Stopping execution");
		0
	}
}
