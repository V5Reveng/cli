use crate::commands::Runnable;

#[derive(clap::Parser)]
pub struct Args {
	#[clap(long, group = "program")]
	name: Option<String>,
	#[clap(long, group = "program")]
	slot: Option<u8>,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		todo!();
	}
}
