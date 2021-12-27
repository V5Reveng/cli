use crate::commands::Runnable;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
	mountpoint: PathBuf,
}

impl Runnable for Args {
	fn run(&mut self) {
		unimplemented!();
	}
}
