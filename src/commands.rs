use crate::device::Device;
use crate::device::UploadableInfo;
use crate::logging;
use crate::presence::Presence;
use clap::Parser;
use std::path::PathBuf;

mod device;
mod filesystem;
mod program;

trait Runnable {
	fn run(&mut self, device: Presence<Device>);
}

#[derive(clap::Subcommand)]
enum Subcommand {
	Filesystem(filesystem::Args),
	Program(program::Args),
	Device(device::Args),
}

impl Runnable for Subcommand {
	fn run(&mut self, dev: Presence<Device>) {
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
	#[clap(long = "verbose", short, parse(from_occurrences))]
	verbosity: usize,
	/// Specify the path to the device. If there is only one device this is not necessary.
	#[cfg_attr(target_family = "unix", doc = "e.g., /dev/ttyACM0")]
	#[cfg_attr(target_family = "windows", doc = "e.g., COM1")]
	#[clap(long = "device", short)]
	device_path: Option<PathBuf>,
	#[clap(subcommand)]
	sub: Subcommand,
}

impl Args {
	fn run(&mut self) {
		logging::set_from_int(self.verbosity);
		let device = if let Some(ref device_path) = self.device_path {
			Presence::One(Device::try_from(device_path.as_ref()).expect("Invalid device provided"))
		} else {
			Presence::from(UploadableInfo::get_all().expect("Failed to get serial ports").into_iter().filter_map(|port| Device::try_from(port).ok()).collect::<Vec<Device>>())
		};
		self.sub.run(device);
	}
}

pub fn run() {
	Args::parse().run();
}
