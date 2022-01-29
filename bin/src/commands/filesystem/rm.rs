use crate::commands::Runnable;
use anyhow::Context;
use v5_device::device::filesystem as fs;

/// Delete a file.
#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: fs::QualFileName,
	/// Whether to delete the file linked to this file, if one exists.
	#[clap(long, short = 'l')]
	include_linked: bool,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		let args = fs::DeleteArgs { include_linked: self.include_linked };
		if dev.delete_file(&self.file, &args).context("Deleting file")? {
			Ok(())
		} else {
			anyhow::bail!("No such file or directory");
		}
	}
}
