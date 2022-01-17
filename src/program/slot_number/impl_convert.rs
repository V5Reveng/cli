use super::{SlotNumber, SlotNumberFromStrError, SlotNumberFromU8Error};
use std::str::FromStr;

impl From<SlotNumber> for u8 {
	fn from(num: SlotNumber) -> Self {
		num.0
	}
}

impl TryFrom<u8> for SlotNumber {
	type Error = SlotNumberFromU8Error;
	fn try_from(num: u8) -> Result<SlotNumber, Self::Error> {
		match num {
			1..=8 => Ok(SlotNumber(num)),
			_ => Err(SlotNumberFromU8Error::OutOfRange),
		}
	}
}

impl FromStr for SlotNumber {
	type Err = SlotNumberFromStrError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		SlotNumber::try_from(u8::from_str(s).map_err(SlotNumberFromStrError::StrToU8)?).map_err(SlotNumberFromStrError::U8ToSlotNumber)
	}
}
