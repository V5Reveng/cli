use crate::commands::Runnable;

#[derive(clap::Parser)]
/// List uploaded programs.
pub struct Args {}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
