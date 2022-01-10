use super::helpers::{LongVersion, Product, ShortVersion};
use encde::Decode;

#[derive(Decode)]
pub struct DeviceInfo {
	pub version: LongVersion,
	#[encde(pad_after = 1)]
	pub product: Product,
}

#[derive(Decode)]
pub struct ExtendedDeviceInfo {
	#[encde(pad_before = 1)]
	pub system_version: ShortVersion,
	pub cpu0_version: ShortVersion,
	#[encde(pad_after = 3)]
	pub cpu1_version: ShortVersion,
	pub touch_version: u8,
	#[encde(pad_after = 12)]
	pub system_id: u32,
}

#[derive(Decode)]
pub struct ExtendedDeviceInfoNew {
	pub common: ExtendedDeviceInfo,
	/// This data is ignored by pros-cli so there's no way to know what it actually is.
	#[encde(pad_after = 3)]
	pub unknown: u8,
}

impl From<ExtendedDeviceInfoNew> for ExtendedDeviceInfo {
	fn from(new: ExtendedDeviceInfoNew) -> Self {
		new.common
	}
}

use super::filesystem::{Address, Category, FileIndex, FileName, FileSize, FileType, TimeStamp};
#[derive(Decode, Debug)]
pub struct FileMetadataByName {
	/// TODO: find out what this field represents
	pub linked_category: Category,
	pub size: FileSize,
	pub addr: Address,
	pub crc: u32,
	pub file_type: FileType,
	pub timestamp: TimeStamp,
	pub version: u32,
	/// TODO: find out what this field represents
	pub linked_name: FileName,
}
#[derive(Decode, Debug)]
pub struct FileMetadataByIndex {
	pub idx: FileIndex,
	pub size: FileSize,
	pub addr: Address,
	pub crc: u32,
	pub file_type: FileType,
	pub timestamp: TimeStamp,
	pub version: u32,
	pub name: FileName,
}

#[derive(Decode)]
pub struct NumFiles(pub i16);
