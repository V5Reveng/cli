use super::{FixedString, FixedStringFromStrError as Error};
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::str::FromStr;

impl<const N: usize> FromStr for FixedString<N> {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() > N {
			return Err(Self::Err::TooLong);
		}
		let mut ret: Self = Self::default();
		for (idx, byte) in s.bytes().chain(std::iter::repeat(0)).take(N).enumerate() {
			ret.0[idx] = byte;
		}
		Ok(ret)
	}
}
impl<const N: usize> TryFrom<&str> for FixedString<N> {
	type Error = <Self as FromStr>::Err;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::from_str(s)
	}
}
impl<const N: usize> TryFrom<&OsStr> for FixedString<N> {
	type Error = Error;
	fn try_from(s: &OsStr) -> Result<Self, Self::Error> {
		Self::from_str(s.to_str().ok_or(Self::Error::InvalidUnicode)?)
	}
}
