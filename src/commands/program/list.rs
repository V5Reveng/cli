use crate::commands::Runnable;
use crate::program;

/// List uploaded programs.
#[derive(clap::Parser)]
pub struct Args {
	#[clap(long, short = 'p')]
	only_present: bool,
}

impl Runnable for Args {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let programs = program::get_all(&mut dev).expect("Getting program list");
		for (idx, program) in programs.iter().enumerate() {
			let idx = program::SlotNumber::from_index(idx).unwrap();
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
		0
	}
}
