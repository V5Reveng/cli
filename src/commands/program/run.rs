use crate::commands::Runnable;

#[derive(clap::Parser)]
/// Remove (a) program(s).
pub struct Args {
	/// Not required if there is only one program, or if the current directory is a project. If the latter is true, the program will only be run, not uploaded; if the program has not yet been uploaded an error will occur.
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
