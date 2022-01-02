use super::helpers::{LongVersion, Padding, Product, ShortVersion};
use bincode::Decode;

#[derive(Decode)]
pub struct DeviceInfo {
	pub version: LongVersion,
	pub product: Product,
	_1: Padding<1>,
}

#[derive(Decode)]
pub struct ExtendedDeviceInfo {
	_1: Padding<1>,
	pub system_version: ShortVersion,
	pub cpu0_version: ShortVersion,
	pub cpu1_version: ShortVersion,
	_2: Padding<3>,
	pub touch_version: u8,
	pub system_id: u32,
	_3: Padding<12>,
}

#[derive(Decode)]
pub struct ExtendedDeviceInfoNew {
	common: ExtendedDeviceInfo,
	/// This data is ignored by pros-cli so there's no way to know what it actually is.
	pub unknown: u8,
	_1: Padding<3>,
}

impl From<ExtendedDeviceInfoNew> for ExtendedDeviceInfo {
	fn from(new: ExtendedDeviceInfoNew) -> Self {
		new.common
	}
}
