use super::filesystem::{Address, Category, FileIndex, FileName, FileSize, FileType, Function, PacketSize, QualFileName, Target, TimeStamp};
use super::helpers::ShortVersion;
use encde::Encode;

#[derive(Encode)]
pub struct FileMetadataByName {
	pub category: Category,
	options: u8,
	pub name: FileName,
}
impl FileMetadataByName {
	pub fn new(data: &QualFileName) -> Self {
		Self {
			category: data.category,
			options: 0,
			name: data.name,
		}
	}
}

#[derive(Encode)]
pub struct FileMetadataByIndex {
	pub index: FileIndex,
	options: u8,
}
impl FileMetadataByIndex {
	pub fn new(index: u8) -> Self {
		Self { index, options: 0 }
	}
}

#[derive(Encode)]
pub struct NumFiles {
	pub category: Category,
	options: u8,
}

impl NumFiles {
	pub fn new(category: Category) -> Self {
		Self { category, options: 0 }
	}
}

#[derive(Encode)]
pub(super) struct StartFileTransfer {
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
pub(super) struct FileTransferRead {
	pub address: Address,
	pub size: PacketSize,
}

#[derive(Encode)]
pub(super) struct DeleteFile {
	pub category: Category,
	options: u8,
	pub name: FileName,
}
impl DeleteFile {
	pub(super) fn new(data: &QualFileName, include_linked: bool) -> Self {
		Self {
			category: data.category,
			options: if include_linked { 0b10_00_00_00 } else { 0 },
			name: data.name,
		}
	}
}
