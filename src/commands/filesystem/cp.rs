use crate::commands::Runnable;

#[derive(clap::Parser)]
pub struct Args {
	/// Remote source filename.
	source: String,
	/// Remote destination filename.
	dest: String,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		todo!();
	}
}
