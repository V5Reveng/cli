use crate::commands::Runnable;
use crate::device::filesystem as fs;
use clap_num::maybe_hex;
use std::io::{stdin, Read};

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: String,
	/// Whether to overwrite the file if it exists
	#[clap(long = "force", short)]
	overwrite: bool,
	/// The address of the file. Only really matters for executables.
	/// If not specified and the remote file exists, use its address.
	/// Otherwise, use a predefined address.
	#[clap(long, parse(try_from_str=maybe_hex))]
	address: Option<fs::Address>,
	/// The category of the file. Can be user, system, pros, rms, mw.
	#[clap(long, short, default_value_t = Default::default())]
	category: fs::Category,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let (file_name, file_type) = crate::commands::string_to_file_name_and_type(&self.file);
		let mut data = Vec::default();
		stdin().read_to_end(&mut data).expect("Could not read from stdin");
		let args = fs::WriteArgs {
			file_name,
			file_type,
			address: self.address,
			overwrite: self.overwrite,
			category: self.category,
			..Default::default()
		};
		dev.write_file_from_slice(&data, &args).unwrap()
	}
}
