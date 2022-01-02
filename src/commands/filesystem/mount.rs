#![cfg(target_os = "linux")]

use crate::commands::Runnable;
use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
	mountpoint: PathBuf,
}

impl Runnable for Args {
	fn run(&mut self, dev: crate::presence::Presence<crate::device::Device>) {
		todo!();
	}
}
