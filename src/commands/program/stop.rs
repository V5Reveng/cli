use crate::commands::Runnable;

/// Stop the running program.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
