#![cfg(target_os = "linux")]

use crate::commands::Runnable;
use std::path::PathBuf;

#[derive(clap::Parser)]
/// Mount the device as a FUSE filesystem.
pub struct Args {
	mount_point: PathBuf,
}

impl Runnable for Args {
	fn run(self, _dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		todo!();
	}
}
