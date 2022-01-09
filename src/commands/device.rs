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
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		self.sub.run(dev);
	}
}

/// Commands that query device information.
#[derive(clap::Subcommand)]
enum Commands {
	/// List connected devices.
	Info(info::Args),
	/// Print device info.
	List(list::Args),
	/// Take a screen capture of the device.
	ScreenCapture(screen_capture::Args),
}

impl super::Runnable for Commands {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		match self {
			Commands::Info(args) => args.run(dev),
			Commands::List(args) => args.run(dev),
			Commands::ScreenCapture(args) => args.run(dev),
		}
	}
}
