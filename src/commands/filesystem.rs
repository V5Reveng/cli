use crate::commands::Runnable;

mod cat;
mod cp;
mod edit;
mod info;
#[cfg(target_os = "linux")]
mod mount;
mod mv;
mod pull;
mod push;
mod rm;

#[derive(clap::Parser)]
pub struct Args {
	#[clap(subcommand)]
	sub: Commands,
}

impl Runnable for Args {
	fn run(&mut self) {
		self.sub.run();
	}
}

/// Filesystem-related commands.
#[derive(clap::Subcommand)]
enum Commands {
	/// Output the contents of a file.
	Cat(cat::Args),
	/// Copy file.
	Cp(cp::Args),
	/// Edit file using $EDITOR.
	Edit(edit::Args),
	/// List files with no argument; print file metadata with an argument.
	Info(info::Args),
	#[cfg(target_os = "linux")]
	/// Mount the device as a FUSE filesystem.
	Mount(mount::Args),
	/// Move a file.
	Mv(mv::Args),
	/// Copy file from device filesystem to local filesystem.
	Pull(pull::Args),
	/// Copy file from local filesystem to device filesystem.
	Push(push::Args),
	/// Delete a file.
	Rm(rm::Args),
}

impl Runnable for Commands {
	fn run(&mut self) {
		match self {
			Commands::Cat(args) => args.run(),
			Commands::Cp(args) => args.run(),
			Commands::Edit(args) => args.run(),
			Commands::Info(args) => args.run(),
			Commands::Mount(args) => args.run(),
			Commands::Mv(args) => args.run(),
			Commands::Pull(args) => args.run(),
			Commands::Push(args) => args.run(),
			Commands::Rm(args) => args.run(),
		}
	}
}
