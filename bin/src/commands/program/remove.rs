use crate::commands::Runnable;
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
	fn run(self, dev: v5_device::util::presence::Presence<v5_device::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		if self.all {
			program::remove_all(&mut dev).expect("Removing all programs");
		} else {
			let program_slots: HashSet<_> = self.program_slots.into_iter().collect();
			for program_slot in program_slots {
				let was_deleted = program::remove(&mut dev, program_slot, false).unwrap_or_else(|_| panic!("Removing slot {}", program_slot));
				if !self.ignore_empty && !was_deleted {
					eprintln!("Slot {} is empty", program_slot);
					return 1;
				}
			}
		}
		0
	}
}
