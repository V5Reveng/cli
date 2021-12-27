use crate::commands::Runnable;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
	#[clap(long, group = "device")]
	name: Option<String>,
	#[clap(long, group = "device")]
	port: Option<PathBuf>,
}

impl Runnable for Args {
	fn run(&mut self) {
		unimplemented!();
	}
}
