use crate::commands::Runnable;

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: String,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		todo!();
	}
}
