use crate::commands::Runnable;

/// Get info for a specific slot.
#[derive(clap::Parser)]
pub struct Args {
	#[clap(long, group = "program")]
	name: Option<String>,
	#[clap(long, group = "program")]
	slot: Option<u8>,
}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
