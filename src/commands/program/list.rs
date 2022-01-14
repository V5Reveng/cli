use crate::commands::Runnable;

/// List uploaded programs.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
