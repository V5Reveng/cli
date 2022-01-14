use crate::commands::Runnable;

#[derive(clap::Parser)]
/// Take a screen capture of the device.
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		todo!();
	}
}
