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
	fn run(&mut self) {
		self.sub.run();
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
	fn run(&mut self) {
		match self {
			Commands::Info(args) => args.run(),
			Commands::List(args) => args.run(),
			Commands::Remove(args) => args.run(),
			Commands::Run(args) => args.run(),
			Commands::Stop(args) => args.run(),
			Commands::Upload(args) => args.run(),
		}
	}
}
