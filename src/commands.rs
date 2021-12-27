use crate::logging;
use clap::Parser;
use std::path::PathBuf;

mod filesystem;
mod program;
mod query;

trait Runnable {
	fn run(&mut self);
}

#[derive(clap::Subcommand)]
enum Subcommand {
	Filesystem(filesystem::Args),
	Program(program::Args),
	Query(query::Args),
}

impl Runnable for Subcommand {
	fn run(&mut self) {
		match self {
			Subcommand::Filesystem(args) => args.run(),
			Subcommand::Program(args) => args.run(),
			Subcommand::Query(args) => args.run(),
		}
	}
}

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
	#[clap(long = "verbose", short, parse(from_occurrences), default_value = "0")]
	verbosity: u32,
	/// Specify the device by name.
	/// If the command doesn't use a device, it will be ignored.
	/// If only one device is connected, specifying the device is not necessary.
	#[clap(long, group = "device")]
	device_name: Option<String>,
	/// Alternatively, specify the path to the device.
	#[cfg_attr(target_family = "unix", doc = "e.g., /dev/ttyACM0")]
	#[cfg_attr(target_family = "windows", doc = "e.g., COM1")]
	#[clap(long, group = "device")]
	device_port: Option<PathBuf>,
	#[clap(subcommand)]
	sub: Subcommand,
}

impl Runnable for Args {
	fn run(&mut self) {
		logging::set_from_int(self.verbosity);
		self.sub.run();
	}
}

pub fn run() {
	let mut args = Args::parse();
	args.run();
}
