use encde::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
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
impl std::fmt::Display for ShortVersion {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(formatter, "{}.{}.{}-{}", self.major, self.minor, self.patch, self.build_major)
	}
}

#[derive(Encode, Decode, PartialEq, Eq, PartialOrd, Ord)]
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
impl std::fmt::Display for LongVersion {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(formatter, "{}.{}", self.common, self.build_minor)
	}
}

impl From<LongVersion> for ShortVersion {
	fn from(long: LongVersion) -> Self {
		long.common
	}
}

impl std::cmp::PartialEq<LongVersion> for ShortVersion {
	fn eq(&self, other: &LongVersion) -> bool {
		self == &other.common
	}
}

impl std::cmp::PartialOrd<LongVersion> for ShortVersion {
	fn partial_cmp(&self, other: &LongVersion) -> Option<std::cmp::Ordering> {
		Some(self.cmp(&other.common))
	}
}

#[derive(Decode)]
pub struct BrainFlags(u8);
impl BrainFlags {
	// empty
}

#[derive(Decode)]
pub struct ControllerFlags(u8);
impl ControllerFlags {
	pub fn connected(&self) -> bool {
		self.0 & 0b10 == 0b10
	}
}

#[derive(Decode)]
#[repr(u8)]
pub enum Product {
	#[encde(wire_tag = 0x10)]
	Brain(BrainFlags),
	#[encde(wire_tag = 0x11)]
	Controller(ControllerFlags),
}

impl std::fmt::Display for Product {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Brain(_) => f.write_str("brain"),
			Self::Controller(flags) => write!(f, "controller (connected: {})", flags.connected()),
		}
	}
}
