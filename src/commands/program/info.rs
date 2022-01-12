use crate::commands::Runnable;

#[derive(clap::Parser)]
/// Get info for a specific slot.
pub struct Args {
	#[clap(long, group = "program")]
	name: Option<String>,
	#[clap(long, group = "program")]
	slot: Option<u8>,
}

impl Runnable for Args {
	fn run(self, _dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
