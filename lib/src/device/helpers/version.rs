//! Read about SemVer for more information about the fields.

use encde::{Decode, Encode};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ShortVersion {
	major: u8,
	minor: u8,
	patch: u8,
	build_major: u8,
}

impl ShortVersion {
	pub fn new(major: u8, minor: u8, patch: u8, build_major: u8) -> Self {
		Self { major, minor, patch, build_major }
	}
}

impl Display for ShortVersion {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}.{}.{}-{}", self.major, self.minor, self.patch, self.build_major)
	}
}

#[derive(Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LongVersion {
	common: ShortVersion,
	build_minor: u8,
}

impl LongVersion {
	pub fn new(major: u8, minor: u8, patch: u8, build_major: u8, build_minor: u8) -> Self {
		Self {
			common: ShortVersion { major, minor, patch, build_major },
			build_minor,
		}
	}
}

impl Display for LongVersion {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}.{}", self.common, self.build_minor)
	}
}

impl From<LongVersion> for ShortVersion {
	fn from(long: LongVersion) -> Self {
		long.common
	}
}

impl PartialEq<LongVersion> for ShortVersion {
	fn eq(&self, other: &LongVersion) -> bool {
		self == &other.common
	}
}

impl PartialOrd<LongVersion> for ShortVersion {
	fn partial_cmp(&self, other: &LongVersion) -> Option<Ordering> {
		Some(self.cmp(&other.common))
	}
}
