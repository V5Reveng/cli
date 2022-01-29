use crate::commands::Runnable;
use anyhow::Context;
use std::collections::HashSet;
use v5_device::program::{self, SlotNumber};

/// Run a program.
#[derive(clap::Parser)]
pub struct Args {
	/// If true, the list is ignored and all programs are removed.
	#[clap(long, short, group = "programs")]
	all: bool,
	/// The program(s) to remove.
	#[clap(group = "programs")]
	program_slots: Vec<SlotNumber>,
	/// Don't complain if one or more of the specified slots is empty.
	#[clap(long, short = 'i')]
	ignore_empty: bool,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		if self.all {
			program::remove_all(&mut dev).context("Removing all programs")?;
		} else {
			let program_slots: HashSet<_> = self.program_slots.into_iter().collect();
			for program_slot in program_slots {
				let was_deleted = program::remove(&mut dev, program_slot, false).context(format!("Removing slot {}", program_slot))?;
				if !self.ignore_empty && !was_deleted {
					anyhow::bail!("Slot {} is empty", program_slot);
				}
			}
		}
		Ok(())
	}
}
