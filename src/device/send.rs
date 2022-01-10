use super::filesystem::{Category, FileIndex, FileName};
use encde::Encode;

#[derive(Encode)]
pub struct FileMetadataByName {
	pub category: Category,
	options: u8,
	pub name: FileName,
}
impl FileMetadataByName {
	pub fn new(category: Category, name: FileName) -> Self {
		Self { category, options: 0, name }
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
