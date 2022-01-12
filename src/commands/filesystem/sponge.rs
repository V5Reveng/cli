use crate::commands::Runnable;
use crate::device::filesystem as fs;
use clap_num::maybe_hex;
use std::io::{stdin, Read};

#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: fs::QualFile,
	/// Whether to overwrite the file if it exists
	#[clap(long = "force", short)]
	overwrite: bool,
	/// The address of the file. Only really matters for executables.
	/// If not specified and the remote file exists, use its address.
	/// Otherwise, use a predefined address.
	#[clap(long, parse(try_from_str=maybe_hex))]
	address: Option<fs::Address>,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let mut data = Vec::default();
		// we have to buffer this to have the size and the CRC
		stdin().read_to_end(&mut data).expect("Could not read from stdin");
		let args = fs::WriteArgs {
			address: self.address,
			overwrite: self.overwrite,
			..Default::default()
		};
		dev.write_file_from_slice(&data, &self.file, &args).unwrap();
		0
	}
}
