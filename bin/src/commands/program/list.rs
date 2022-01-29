use crate::commands::Runnable;
use anyhow::Context;
use v5_device::program;

/// List uploaded programs.
#[derive(clap::Parser)]
pub struct Args {
	#[clap(long, short = 'p')]
	only_present: bool,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		let programs = program::get_all(&mut dev).context("Getting program list")?;
		for (idx, program) in programs.iter().enumerate() {
			let idx = program::SlotNumber::from_index(idx)?;
			match program.as_ref() {
				Some(program) => {
					println!("Slot {}: {}", idx, program.name);
				}
				None if !self.only_present => {
					println!("Slot {}: (none)", idx);
				}
				None => (),
			}
		}
		Ok(())
	}
}
