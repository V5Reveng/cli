use crate::commands::Runnable;
use anyhow::Context;
use clap_num::maybe_hex;
use std::io::{stdin, Read};
use v5_device::device::filesystem as fs;

/// Write stdin to a remote file.
///
/// To "push" a file to the device, you can add ` < local.file` to the command line.
#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: fs::QualFile,
	/// Whether to overwrite the file if it exists
	#[clap(long = "force", short)]
	overwrite: bool,
	/// The address of the file. (Expert)
	///
	/// Only really matters for executables.
	/// If not specified and the remote file exists, use its address.
	/// Otherwise, use a predefined address.
	#[clap(long, parse(try_from_str=maybe_hex))]
	address: Option<fs::Address>,
	/// The link of the file. (Expert)
	///
	/// If file A has a link to file B, then B is loaded into memory along with A when A is executed.
	#[clap(long)]
	link: Option<fs::QualFileName>,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		let mut data = Vec::default();
		// we have to buffer this to have the size and the CRC
		stdin().read_to_end(&mut data).context("Could not read from stdin")?;
		let args = fs::WriteArgs {
			address: self.address,
			overwrite: self.overwrite,
			linked_file: self.link,
			..Default::default()
		};
		dev.write_file_from_slice(&data, &self.file, &args).context("Writing file")
	}
}
