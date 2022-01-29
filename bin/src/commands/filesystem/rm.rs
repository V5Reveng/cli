use crate::commands::Runnable;
use v5_device::device::{filesystem as fs, Device};

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
	fn run(self, dev: v5_device::util::presence::Presence<Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let args = fs::DeleteArgs { include_linked: self.include_linked };
		if dev.delete_file(&self.file, &args).unwrap() {
			0
		} else {
			eprintln!("No such file or directory");
			1
		}
	}
}
