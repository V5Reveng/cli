use super::FixedString;
use std::fmt::{self, Error, Formatter, Result};

impl<const N: usize> fmt::Debug for FixedString<N> {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		write!(formatter, "FixedString<{}> ", N)?;
		self.as_str().map_err(|_| Error::default())?.fmt(formatter)
	}
}

impl<const N: usize> fmt::Display for FixedString<N> {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		self.as_str().map_err(|_| Error::default())?.fmt(formatter)
	}
}
