use crate::commands::Runnable;
use anyhow::Context;
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
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		if self.raw {
			let file = QualFileName::from_str(&self.slot).context("Filename")?;
			dev.execute_file(&file).context("Running file")?;
		} else {
			let slot = SlotNumber::from_str(&self.slot).context("Slot number")?;
			program::run(&mut dev, slot).context("Running program")?;
		}
		Ok(())
	}
}
