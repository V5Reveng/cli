use crate::commands::Runnable;
use std::str::FromStr;
use v5_device::device::filesystem::QualFileName;
use v5_device::program::{self, SlotNumber};

/// Run a program.
#[derive(clap::Parser)]
pub struct Args {
	/// If specified, the "slot" argument will be interpreted as a qualified filename rather than a slot number.
	#[clap(long, short)]
	raw: bool,
	/// The slot number (or qualified filename if `--raw`) to execute.
	slot: String,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence<v5_device::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		if self.raw {
			let file = QualFileName::from_str(&self.slot).expect("Invalid filename");
			dev.execute_file(&file).expect("Running file");
		} else {
			let slot = SlotNumber::from_str(&self.slot).expect("Invalid slot number");
			program::run(&mut dev, slot).expect("Running program");
		}
		0
	}
}
