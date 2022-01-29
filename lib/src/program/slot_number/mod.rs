use std::fmt::{self, Display, Formatter};

pub mod error;
mod impl_convert;
mod impl_serde;

pub use error::{SlotNumberFromStrError, SlotNumberFromU8Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SlotNumber(u8);

impl SlotNumber {
	pub fn from_index(idx: usize) -> Result<Self, SlotNumberFromU8Error> {
		Self::try_from(u8::try_from(idx + 1).map_err(|_| SlotNumberFromU8Error::OutOfRange)?)
	}
	pub fn to_index(self) -> usize {
		(self.0 - 1) as usize
	}
}

impl Display for SlotNumber {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		self.0.fmt(formatter)
	}
}
