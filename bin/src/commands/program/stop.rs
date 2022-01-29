use crate::commands::Runnable;
use anyhow::Context;

/// Stop the running program.
#[derive(clap::Parser)]
pub struct Args {}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		dev.stop_execution().context("Stopping execution")
	}
}
