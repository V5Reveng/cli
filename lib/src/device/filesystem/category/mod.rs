use encde::{Decode, Encode};

pub mod error;
pub mod impl_display;
pub mod impl_from_str;

pub use error::CategoryFromStrError;

/// The category of a file.
///
/// This is a primitive way to create subdirectories on the device's filesystem.
/// Each category acts as one subdirectory of the root directory.
#[derive(Encode, Decode, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct Category(pub u8);

impl Category {
	pub const NONE: Category = Category(0);
	pub const USER: Category = Category(1);
	pub const SYSTEM: Category = Category(15);
	pub const RMS: Category = Category(16);
	pub const PROS: Category = Category(24);
	pub const MW: Category = Category(32);
	pub const REVENG: Category = Category(48);

	pub const MIN: u8 = u8::MIN;
	pub const MAX: u8 = u8::MAX;

	pub const fn named() -> &'static [Self] {
		&[Self::USER, Self::SYSTEM, Self::RMS, Self::PROS, Self::MW, Self::REVENG]
	}

	pub fn is_none(&self) -> bool {
		*self == Self::NONE
	}

	pub fn into_inner(self) -> u8 {
		self.0
	}
}

impl Default for Category {
	fn default() -> Self {
		Self::USER
	}
}
