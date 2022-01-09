use crate::commands::Runnable;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
	/// Local source path.
	local: PathBuf,
	/// Remote destination filename.
	/// Defaults to the basename of the local path.
	remote: Option<String>,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		todo!();
	}
}
