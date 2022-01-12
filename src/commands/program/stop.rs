use crate::commands::Runnable;

#[derive(clap::Parser)]
/// Stop the running program.
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
