use crate::commands::Runnable;
use anyhow::Context;
use v5_device::program::{get as get_program, ProgramIni, SlotNumber};

/// Get info for a specific slot.
#[derive(clap::Parser)]
pub struct Args {
	/// Slot number, from 1 to 8.
	slot: SlotNumber,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		let program = get_program(&mut dev, self.slot).context("Getting program")?;
		match program {
			Some(ref program) => {
				print_program(program);
				Ok(())
			}
			None => {
				anyhow::bail!("Program in slot {} does not exist", self.slot);
			}
		}
	}
}

fn print_program(ProgramIni { version, name, slot, icon, description, date }: &ProgramIni) {
	println!("Name: {}", name);
	println!("Version: {}", version);
	println!("Slot: {}", slot);
	println!("Icon: {}", icon);
	println!("Description: {}", description);
	println!("Date: {}", date);
}
