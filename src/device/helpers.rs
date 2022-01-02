use bincode::{
	de::Decoder,
	error::{AllowedEnumVariants, DecodeError},
	Decode, Encode,
};

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

pub enum Product {
	/// Identified by 0x10
	Brain,
	/// Identified by 0x11
	Controller {
		/// If the controller is connected to a brain.
		connected: bool,
	},
}

impl Decode for Product {
	fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
		// [product, flags]
		let data = <[u8; 2]>::decode(decoder)?;
		match data[0] {
			0x10 => Ok(Product::Brain), // flags is ignored
			0x11 => Ok(Product::Controller {
				connected: crate::util::has_bit(data[1].into(), 1),
			}),
			found => Err(DecodeError::UnexpectedVariant {
				type_name: "Product",
				allowed: AllowedEnumVariants::Allowed(&[0x10, 0x11]),
				found: found.into(),
			}),
		}
	}
}

impl std::fmt::Display for Product {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Brain => f.write_str("brain"),
			Self::Controller { connected } => write!(f, "controller (connected: {})", connected),
		}
	}
}

// https://stackoverflow.com/a/64651623
/// A zero-sized type that will be decoded as if it has the size `SIZE`, but without keeping the data. That is, it skips `SIZE` bytes in the stream.
///
/// For example:
///
/// ```
/// use bincode::Decode;
/// #[derive(Decode)]
/// struct Something {
/// 	actual_field: u32,
/// 	_1: Padding<3>,
/// 	another_actual_field: u8,
/// }
/// ```
///
/// This declares that there are three bytes of padding after the u32 and before the u8.
pub struct Padding<const SIZE: usize>(());
impl<const SIZE: usize> Decode for Padding<SIZE> {
	fn decode<D: Decoder>(decoder: D) -> Result<Self, DecodeError> {
		<[u8; SIZE]>::decode(decoder)?;
		Ok(Padding(()))
	}
}

pub fn config() -> bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::SkipFixedArrayLength, bincode::config::NoLimit> {
	bincode::config::Configuration::standard().with_little_endian().with_fixed_int_encoding().skip_fixed_array_length().with_no_limit()
}
