//! The user interface on the command line.

use crate::logging;
use clap::Parser;
use std::path::PathBuf;
use v5_device::device::{Device, UploadableInfo};
use v5_device::util::presence::Presence;

mod device;
mod filesystem;
mod program;

/// A command that can be run with an arbitrary number of devices present (none, one, or many).
trait Runnable {
	fn run(self, device: Presence<Device>) -> u32;
}

#[derive(clap::Subcommand)]
enum Subcommand {
	Filesystem(filesystem::Args),
	Program(program::Args),
	Device(device::Args),
}

impl Runnable for Subcommand {
	fn run(self, dev: Presence<Device>) -> u32 {
		match self {
			Subcommand::Filesystem(args) => args.run(dev),
			Subcommand::Program(args) => args.run(dev),
			Subcommand::Device(args) => args.run(dev),
		}
	}
}

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
	/// Increase verbosity.
	#[clap(long = "verbose", short, parse(from_occurrences))]
	verbosity: usize,
	/// Specify the path to the device
	#[cfg_attr(target_family = "unix", doc = "e.g., /dev/ttyACM0.")]
	#[cfg_attr(target_family = "windows", doc = "e.g., COM1.")]
	/// Not necessary if there is only one device.
	#[clap(long = "device", short)]
	device_path: Option<PathBuf>,
	#[clap(subcommand)]
	sub: Subcommand,
}

impl Args {
	fn run(self) -> u32 {
		if self.verbosity > 0 {
			logging::set_from_int(self.verbosity);
		}
		let device = if let Some(ref device_path) = self.device_path {
			Presence::One(Device::try_from(device_path.as_ref()).expect("Invalid device provided"))
		} else {
			Presence::from(UploadableInfo::get_all().expect("Failed to get serial ports").into_iter().filter_map(|port| Device::try_from(port).ok()).collect::<Vec<Device>>())
		};
		self.sub.run(device)
	}
}

pub fn run() -> u32 {
	Args::parse().run()
}

/// Supplies standard messages to the `expect_one` method on a `Presence` instance.
pub fn unwrap_device_presence(pres: Presence<Device>) -> Device {
	pres.expect_one("No uploadable devices found.", "Multiple uploadable devices found. You can specify just one with the --device-path argument.")
}
