use crate::commands::Runnable;

/// Take a screen capture of the device.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		todo!();
	}
}
