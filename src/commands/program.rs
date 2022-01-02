use crate::commands::Runnable;

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
	fn run(&mut self, dev: crate::presence::Presence<crate::device::Device>) {
		self.sub.run(dev);
	}
}

/// Program-related commands.
#[derive(clap::Subcommand)]
enum Commands {
	/// Get info for a specific slot.
	Info(info::Args),
	/// List uploaded programs.
	List(list::Args),
	/// Remove (a) program(s).
	Run(run::Args),
	/// Run a program.
	Remove(remove::Args),
	/// Stop the running program.
	Stop(stop::Args),
	/// Upload a program.
	Upload(upload::Args),
}

impl Runnable for Commands {
	fn run(&mut self, dev: crate::presence::Presence<crate::device::Device>) {
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
