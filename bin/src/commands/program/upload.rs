use crate::commands::Runnable;

/// Upload a program.
#[derive(clap::Parser)]
pub struct Args {
	/// Optionally override the name of the program when it's uploaded.
	/// Defaults to the project name in Cargo.toml.
	name: Option<String>,
	/// Optionally specify the slot to upload to.
	/// If not specified, uses the first non-empty slot, unless there is a program already uploaded with the same name and that is older than this version, in which case that slot is used.
	#[clap(short, long)]
	slot: Option<u8>,
	/// When slot is specified, overwrite the slot's contents if it is already occupied.
	#[clap(short, long, requires = "slot")]
	force: bool,
}

impl Runnable for Args {
	fn run(self, _dev: v5_device::util::presence::Presence<v5_device::device::Device>) -> u32 {
		todo!()
	}
}
