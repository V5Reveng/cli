use std::fmt::{self, Display, Formatter};

/// Variants are documented in the Display implementation.
#[derive(Debug)]
pub enum FixedStringFromStrError {
	TooLong,
	ContainsNul { position: usize },
}

impl Display for FixedStringFromStrError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self {
			Self::TooLong => formatter.write_str("string is too long"),
			Self::ContainsNul { position } => write!(formatter, "string contains a NUL byte at position {}", position),
		}
	}
}

impl std::error::Error for FixedStringFromStrError {}
