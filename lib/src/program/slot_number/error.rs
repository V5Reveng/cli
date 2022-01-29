use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotNumberFromU8Error {
	OutOfRange,
}

impl Display for SlotNumberFromU8Error {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self {
			Self::OutOfRange => write!(formatter, "value is not in the range 1 to 8 inclusive"),
		}
	}
}

impl Error for SlotNumberFromU8Error {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotNumberFromStrError {
	StrToU8(ParseIntError),
	U8ToSlotNumber(SlotNumberFromU8Error),
}

impl Display for SlotNumberFromStrError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self {
			Self::StrToU8(e) => write!(formatter, "parsing to u8: {}", e),
			Self::U8ToSlotNumber(e) => write!(formatter, "converting u8 to slot number: {}", e),
		}
	}
}

impl Error for SlotNumberFromStrError {}
