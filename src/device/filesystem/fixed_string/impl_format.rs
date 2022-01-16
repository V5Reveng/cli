use super::FixedString;
use std::fmt::{self, Formatter, Result};

impl<const N: usize> fmt::Debug for FixedString<N> {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		write!(formatter, "FixedString<{}> \"{:?}\"", N, self.as_bytes())
	}
}

impl<const N: usize> fmt::Display for FixedString<N> {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		match self.as_str() {
			Ok(s) => s.fmt(formatter),
			Err(_) => write!(formatter, "{:?}", self.as_bytes()),
		}
	}
}
