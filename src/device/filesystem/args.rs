//! Structures used as default arguments for public filesystem-related `Device` methods.

use super::{Address, FileSize, QualFileName, Target, TimeStamp, TransferCompleteAction};

/// Extra arguments to `read_file_to_stream`.
#[derive(Default)]
pub struct ReadArgs {
	pub target: Target,
	/// If not specified, read from the start.
	pub address: Option<Address>,
	/// If not specified, read the entire file.
	pub size: Option<FileSize>,
}

/// Extra arguments to `write_file_from_{stream,slice}`.
#[derive(Default)]
pub struct WriteArgs {
	/// Only applies to executables.
	pub action: TransferCompleteAction,
	pub target: Target,
	/// If not specified, use the file's address if the file exists, otherwise `DEFAULT_ADDRESS`.
	pub address: Option<Address>,
	pub overwrite: bool,
	pub timestamp: TimeStamp,
	/// If specified, link to the specified file.
	pub linked_file: Option<QualFileName>,
}

/// Extra arguments to `delete_file`.
#[derive(Default)]
pub struct DeleteArgs {
	/// Whether to also delete the linked file, if the file has a link.
	pub include_linked: bool,
}
