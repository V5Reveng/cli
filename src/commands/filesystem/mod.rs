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
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		self.sub.run(dev)
	}
}

/// Interact with the filesystem.
#[derive(clap::Subcommand)]
enum Commands {
	Cat(cat::Args),
	Edit(edit::Args),
	Info(info::Args),
	Ls(ls::Args),
	#[cfg(target_os = "linux")]
	Mount(mount::Args),
	Rm(rm::Args),
	Sponge(sponge::Args),
}

impl Runnable for Commands {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		match self {
			Commands::Cat(args) => args.run(dev),
			Commands::Edit(args) => args.run(dev),
			Commands::Info(args) => args.run(dev),
			Commands::Ls(args) => args.run(dev),
			#[cfg(target_os = "linux")]
			Commands::Mount(args) => args.run(dev),
			Commands::Rm(args) => args.run(dev),
			Commands::Sponge(args) => args.run(dev),
		}
	}
}
