use crate::commands::Runnable;

#[derive(clap::Parser)]
pub struct Args {
	// no args
}

impl Runnable for Args {
	fn run(self, _dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
