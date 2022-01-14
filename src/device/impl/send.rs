use crate::device::filesystem::*;
use crate::device::helpers::ShortVersion;
use encde::Encode;

#[derive(Encode)]
pub struct StartFileTransfer {
	pub function: Function,
	pub target: Target,
	pub category: Category,
	pub overwrite: bool,
	pub size: FileSize,
	pub address: Address,
	pub crc: u32,
	pub file_type: FileType,
	pub timestamp: TimeStamp,
	pub version: ShortVersion,
	pub name: FileName,
}

#[derive(Encode)]
pub struct FileTransferRead {
	pub address: Address,
	pub size: PacketSize,
}

#[derive(Encode)]
pub struct DeleteFile {
	pub category: Category,
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

#[derive(Encode)]
pub struct FileTransferSetLink {
	pub linked_category: Category,
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
