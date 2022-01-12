use crate::commands::Runnable;

mod cat;
mod edit;
mod info;
mod ls;
#[cfg(target_os = "linux")]
mod mount;
mod rm;
mod sponge;

#[derive(clap::Parser)]
pub struct Args {
	#[clap(subcommand)]
	sub: Commands,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		self.sub.run(dev)
	}
}

/// Filesystem-related commands.
#[derive(clap::Subcommand)]
enum Commands {
	/// Output the contents of a file.
	/// To "pull" a file from the device, you can add ` > local.file` to the command line.
	Cat(cat::Args),
	/// Edit file using $EDITOR.
	Edit(edit::Args),
	/// Print file metadata.
	Info(info::Args),
	/// List files in a category, or all files.
	/// Note: omitting the category will only list files in named categories.
	Ls(ls::Args),
	#[cfg(target_os = "linux")]
	/// Mount the device as a FUSE filesystem.
	Mount(mount::Args),
	/// Write stdin to a remote file.
	/// To "push" a file to the device, you can add ` < local.file` to the command line.
	Sponge(sponge::Args),
	/// Delete a file.
	Rm(rm::Args),
}

impl Runnable for Commands {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		match self {
			Commands::Cat(args) => args.run(dev),
			Commands::Edit(args) => args.run(dev),
			Commands::Info(args) => args.run(dev),
			Commands::Ls(args) => args.run(dev),
			Commands::Mount(args) => args.run(dev),
			Commands::Rm(args) => args.run(dev),
			Commands::Sponge(args) => args.run(dev),
		}
	}
}
