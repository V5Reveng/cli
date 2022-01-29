#![cfg(target_os = "linux")]

use crate::commands::Runnable;
use std::path::PathBuf;

/// Mount the device as a FUSE filesystem.
#[derive(clap::Parser)]
pub struct Args {
	mount_point: PathBuf,
}

impl Runnable for Args {
	fn run(self, _dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		todo!();
	}
}
