use crate::commands::Runnable;

#[derive(clap::Parser)]
pub struct Args {
	// no args
}

impl Runnable for Args {
	fn run(&mut self) {
		unimplemented!();
	}
}
