//! Payloads to be sent with commands. These are private to the `Device`, as opposed to `super::send` which is public to the crate.

use crate::device::filesystem::*;
use crate::device::helpers::ShortVersion;
use encde::Encode;

/// Start a file transfer.
#[derive(Encode)]
pub struct StartFileTransfer {
	pub function: Function,
	pub target: Target,
	pub category: Category,
	/// Upload only: whether to replace the file if it already exists.
	pub overwrite: bool,
	/// Upload only
	pub size: FileSize,
	pub address: Address,
	/// Upload only
	pub crc: u32,
	pub file_type: FileType,
	/// Upload only
	pub timestamp: TimeStamp,
	/// Upload only
	pub version: ShortVersion,
	pub name: FileName,
}

/// Read a packet at `address` with size `size`.
#[derive(Encode)]
pub struct FileTransferRead {
	pub address: Address,
	/// Must be less than the max packet size received when starting the transfer.
	pub size: PacketSize,
}

#[derive(Encode)]
pub struct DeleteFile {
	pub category: Category,
	/// The MSB indicates whether to also delete the linked file (if the file has one).
	options: u8,
	pub name: FileName,
}

impl DeleteFile {
	pub fn new(data: &QualFileName, include_linked: bool) -> Self {
		Self {
			category: data.category,
			options: if include_linked { 0b10_00_00_00 } else { 0 },
			name: data.name,
		}
	}
}

/// Set the link of a file. For unknown reasons, this can only be set during a file transfer.
#[derive(Encode)]
pub struct FileTransferSetLink {
	pub linked_category: Category,
	/// Currently unused.
	options: u8,
	pub linked_name: FileName,
}

impl FileTransferSetLink {
	pub fn new(linked_file: &QualFileName) -> Self {
		Self {
			linked_category: linked_file.category,
			options: 0,
			linked_name: linked_file.name,
		}
	}
}

#[derive(Encode)]
pub struct FileTransferSetChannel {
	/// Currently unused.
	options: u8,
	pub channel: Channel,
}

impl FileTransferSetChannel {
	pub fn new(channel: Channel) -> Self {
		Self {
			options: 1, // mimic PROS CLI
			channel,
		}
	}
}

#[derive(Encode)]
pub struct ExecuteFile {
	pub category: Category,
	options: u8,
	pub name: FileName,
}

impl ExecuteFile {
	pub fn start(file: &QualFileName) -> Self {
		Self {
			category: file.category,
			options: 0,
			name: file.name,
		}
	}
	pub fn stop() -> Self {
		Self {
			category: Default::default(),
			options: 0x80,
			name: Default::default(),
		}
	}
}
