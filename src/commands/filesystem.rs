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
	fn run(&mut self, dev: crate::presence::Presence<crate::device::Device>) {
		self.sub.run(dev);
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
	fn run(&mut self, dev: crate::presence::Presence<crate::device::Device>) {
		match self {
			Commands::Cat(args) => args.run(dev),
			Commands::Cp(args) => args.run(dev),
			Commands::Edit(args) => args.run(dev),
			Commands::Info(args) => args.run(dev),
			Commands::Mount(args) => args.run(dev),
			Commands::Mv(args) => args.run(dev),
			Commands::Pull(args) => args.run(dev),
			Commands::Push(args) => args.run(dev),
			Commands::Rm(args) => args.run(dev),
		}
	}
}
