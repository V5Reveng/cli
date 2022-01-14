use crate::commands::Runnable;
use crate::device::filesystem as fs;

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
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let args = fs::DeleteArgs { include_linked: self.include_linked };
		dev.delete_file(&self.file, &args).unwrap();
		0
	}
}
