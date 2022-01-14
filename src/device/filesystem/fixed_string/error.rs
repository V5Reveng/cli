use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum FixedStringFromStrError {
	TooLong,
	InvalidUnicode,
}

impl Display for FixedStringFromStrError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		let s = match self {
			Self::TooLong => "string is too long",
			Self::InvalidUnicode => "string is invalid Unicode",
		};
		formatter.write_str(s)
	}
}

impl std::error::Error for FixedStringFromStrError {}
