use crate::commands::Runnable;
use crate::device::filesystem::QualFileName;
use crate::program::{self, SlotNumber};
use std::str::FromStr;

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
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
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
