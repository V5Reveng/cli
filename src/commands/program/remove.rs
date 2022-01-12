use crate::commands::Runnable;

#[derive(clap::Parser)]
pub struct Args {
	/// If true, the list is ignored.
	#[clap(long)]
	all: bool,
	program_names: Vec<String>,
}

impl Runnable for Args {
	fn run(self, _dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
