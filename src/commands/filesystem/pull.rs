use crate::commands::Runnable;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
	/// Remote source filename.
	remote: String,
	/// Local destination path.
	/// Defaults to the remote name, in the current directory.
	local: Option<PathBuf>,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		todo!();
	}
}
