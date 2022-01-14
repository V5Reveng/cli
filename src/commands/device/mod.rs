use crate::commands::Runnable;

mod info;
mod list;
mod screen_capture;

#[derive(clap::Parser)]
pub struct Args {
	#[clap(subcommand)]
	sub: Commands,
}

impl Runnable for Args {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
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

impl super::Runnable for Commands {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		match self {
			Commands::Info(args) => args.run(dev),
			Commands::List(args) => args.run(dev),
			Commands::ScreenCapture(args) => args.run(dev),
		}
	}
}
