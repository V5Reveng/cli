use crate::commands::Runnable;

/// Run a program.
#[derive(clap::Parser)]
pub struct Args {
	/// If true, the list is ignored and all programs are removed.
	#[clap(long)]
	all: bool,
	/// The program(s) to remove.
	program_names: Vec<String>,
}

impl Runnable for Args {
	fn run(self, _dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		todo!()
	}
}
