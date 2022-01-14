use encde::{Decode, Encode};

#[derive(Debug)]
pub enum FixedStringFromStrError {
	TooLong,
	InvalidUnicode,
}
impl std::fmt::Display for FixedStringFromStrError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let s = match self {
			Self::TooLong => "string is too long",
			Self::InvalidUnicode => "string is invalid Unicode",
		};
		formatter.write_str(s)
	}
}
impl std::error::Error for FixedStringFromStrError {}

#[derive(Encode, Decode, Clone, Copy, Eq)]
#[repr(transparent)]
pub struct FixedString<const N: usize>([u8; N]);
impl<const N: usize> Default for FixedString<N> {
	fn default() -> Self {
		Self([0u8; N])
	}
}
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
impl<const N: usize> FixedString<N> {
	pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
		let mut len = N;
		for (idx, &byte) in self.0.iter().enumerate() {
			if byte == 0 {
				len = idx;
				break;
			}
		}
		std::str::from_utf8(&self.0[..len])
	}
}
impl<const N: usize> std::fmt::Debug for FixedString<N> {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "FixedString<{}> ", N)?;
		self.as_str().map_err(|_| std::fmt::Error::default())?.fmt(formatter)
	}
}
impl<const N: usize> std::fmt::Display for FixedString<N> {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.as_str().map_err(|_| std::fmt::Error::default())?.fmt(formatter)
	}
}
impl<const N: usize> PartialEq for FixedString<N> {
	fn eq(&self, other: &Self) -> bool {
		for (&c1, &c2) in self.0.iter().zip(other.0.iter()) {
			if c1 != c2 {
				return false;
			}
			// end of string: quit early
			// c1 == c2 due to previous block
			if c1 == 0 {
				return true;
			}
		}
		true
	}
}
impl<const N: usize> std::hash::Hash for FixedString<N> {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		for c in self.0 {
			if c == 0 {
				break;
			}
			hasher.write_u8(c);
		}
	}
}
