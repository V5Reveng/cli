use crate::commands::Runnable;
use v5_device::util::presence::Presence;

mod info;
mod list;
mod screen_capture;

#[derive(clap::Parser)]
pub struct Args {
	#[clap(subcommand)]
	sub: Commands,
}

impl Runnable for Args {
	fn run(self, dev: Presence) -> anyhow::Result<()> {
		self.sub.run(dev)
	}
}

/// Query device information.
#[derive(clap::Subcommand)]
enum Commands {
	Info(info::Args),
	List(list::Args),
	ScreenCapture(screen_capture::Args),
}

impl Runnable for Commands {
	fn run(self, dev: Presence) -> anyhow::Result<()> {
		match self {
			Commands::Info(args) => args.run(dev),
			Commands::List(args) => args.run(dev),
			Commands::ScreenCapture(args) => args.run(dev),
		}
	}
}
