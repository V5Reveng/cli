use crate::commands::Runnable;
use v5_device::device::Device;
use v5_device::util::presence::Presence;

mod info;
mod list;
mod remove;
mod run;
mod stop;
mod upload;

#[derive(clap::Parser)]
pub struct Args {
	#[clap(subcommand)]
	sub: Commands,
}

impl super::Runnable for Args {
	fn run(self, dev: Presence<Device>) -> u32 {
		self.sub.run(dev)
	}
}

/// Interact with programs and execution.
#[derive(clap::Subcommand)]
enum Commands {
	Info(info::Args),
	List(list::Args),
	Remove(remove::Args),
	Run(run::Args),
	Stop(stop::Args),
	Upload(upload::Args),
}

impl Runnable for Commands {
	fn run(self, dev: Presence<Device>) -> u32 {
		match self {
			Commands::Info(args) => args.run(dev),
			Commands::List(args) => args.run(dev),
			Commands::Remove(args) => args.run(dev),
			Commands::Run(args) => args.run(dev),
			Commands::Stop(args) => args.run(dev),
			Commands::Upload(args) => args.run(dev),
		}
	}
}
