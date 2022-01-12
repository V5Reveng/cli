use crate::commands::Runnable;
use crate::device::filesystem as fs;

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: String,
	/// The category of the file. Can be user, system, pros, rms, mw
	#[clap(long, short, default_value_t = Default::default())]
	category: fs::Category,
	/// Whether to delete the file linked to this file, if one exists.
	#[clap(long, short = 'l')]
	include_linked: bool,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let (file_name, _) = crate::commands::string_to_file_name_and_type(&self.file);
		let args = fs::DeleteArgs {
			category: self.category,
			include_linked: self.include_linked,
		};
		dev.delete_file(&file_name, &args).unwrap();
		0
	}
}
