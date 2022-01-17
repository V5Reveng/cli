use crate::commands::Runnable;
use crate::program::{get as get_program, ProgramIni, SlotNumber};

/// Get info for a specific slot.
#[derive(clap::Parser)]
pub struct Args {
	/// Slot number, from 1 to 8.
	slot: SlotNumber,
}

impl Runnable for Args {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let program = get_program(&mut dev, self.slot).expect("Getting program");
		match program {
			Some(ref program) => {
				print_program(program);
				0
			}
			None => {
				eprintln!("Program in slot {} does not exist", self.slot);
				1
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
