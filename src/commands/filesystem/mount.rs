#![cfg(target_os = "linux")]

use crate::commands::Runnable;
use std::path::PathBuf;

/// Mount the device as a FUSE filesystem.
#[derive(clap::Parser)]
pub struct Args {
	mount_point: PathBuf,
}

impl Runnable for Args {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		let dev = crate::commands::unwrap_device_presence(dev);
		fuse::mount(dev.into_fuse(), &self.mount_point, &[]).expect("Could not mount filesystem");
		0
	}
}
