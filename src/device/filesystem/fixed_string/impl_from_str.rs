use super::{FixedString, FixedStringFromStrError};

impl<const N: usize> std::str::FromStr for FixedString<N> {
	type Err = FixedStringFromStrError;
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
impl<const N: usize> std::convert::TryFrom<&str> for FixedString<N> {
	type Error = <Self as std::str::FromStr>::Err;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		<Self as std::str::FromStr>::from_str(s)
	}
}
impl<const N: usize> std::convert::TryFrom<&std::ffi::OsStr> for FixedString<N> {
	type Error = FixedStringFromStrError;
	fn try_from(s: &std::ffi::OsStr) -> Result<Self, Self::Error> {
		<Self as std::str::FromStr>::from_str(s.to_str().ok_or(Self::Error::InvalidUnicode)?)
	}
}
