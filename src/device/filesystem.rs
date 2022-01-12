use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use encde::{Decode, Encode};
use std::io;

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Function {
	Upload = 1,
	Download = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Category {
	None,
	User,
	System,
	Rms,
	Pros,
	Mw,
	Unnamed(u8),
}
impl Encode for Category {
	fn encode(&self, writer: &mut dyn std::io::Write) -> encde::Result<()> {
		let value = u8::from(*self);
		value.encode(writer)
	}
}
impl From<u8> for Category {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::None,
			1 => Self::User,
			15 => Self::System,
			16 => Self::Rms,
			24 => Self::Pros,
			32 => Self::Mw,
			x => Self::Unnamed(x),
		}
	}
}
impl From<Category> for u8 {
	fn from(value: Category) -> Self {
		match value {
			Category::None => 0,
			Category::User => 1,
			Category::System => 15,
			Category::Rms => 16,
			Category::Pros => 24,
			Category::Mw => 32,
			Category::Unnamed(x) => x,
		}
	}
}
impl Decode for Category {
	fn decode(reader: &mut dyn std::io::Read) -> encde::Result<Self> {
		let value = u8::decode(reader)?;
		Ok(Self::from(value))
	}
}
impl Default for Category {
	fn default() -> Self {
		Self::User
	}
}
impl std::fmt::Display for Category {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let name = match self {
			Self::None => "(none)",
			Self::User => "user",
			Self::System => "system",
			Self::Rms => "RobotMesh Studio",
			Self::Pros => "PROS",
			Self::Mw => "MW",
			Self::Unnamed(x) => {
				return write!(formatter, "0x{:02x}", x);
			}
		};
		formatter.write_str(name)
	}
}
impl std::str::FromStr for Category {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"default" => Ok(Self::default()),
			"user" => Ok(Self::User),
			"system" => Ok(Self::System),
			"rms" => Ok(Self::Rms),
			"pros" => Ok(Self::Pros),
			"mw" => Ok(Self::Mw),
			s => match u8::from_str(s) {
				Ok(x) => Ok(Self::Unnamed(x)),
				Err(_) => Err("Unknown file category. Possible categories are default (user), user, system, rms, pros, mw, or an unnamed category as a number."),
			},
		}
	}
}

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
	Pit = 0,
	Download = 1,
}

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Target {
	Ddr = 0,
	Flash = 1,
	Screen = 2,
}
impl Default for Target {
	fn default() -> Self {
		Self::Flash
	}
}

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransferCompleteAction {
	NoRun = 0b00,
	RunImmediately = 0b01,
	RunScreen = 0b11,
}
impl Default for TransferCompleteAction {
	fn default() -> Self {
		Self::NoRun
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct TimeStamp(DateTime<Local>);
impl TimeStamp {
	pub fn now() -> Self {
		Self(Local::now())
	}
	fn as_repr(&self) -> Result<u32, std::num::TryFromIntError> {
		let base_time = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
		(self.0 - base_time).num_seconds().try_into()
	}
	fn from_repr(repr: u32) -> Option<Self> {
		let base_time = NaiveDateTime::new(NaiveDate::from_ymd(2000, 1, 1), NaiveTime::from_hms(0, 0, 0));
		let base_time = Local.from_local_datetime(&base_time).single()?;
		Some(Self(base_time + Duration::seconds(repr as i64)))
	}
}
impl std::fmt::Display for TimeStamp {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.fmt(formatter)
	}
}
impl Encode for TimeStamp {
	fn encode(&self, writer: &mut dyn io::Write) -> encde::Result<()> {
		self.as_repr().map_err(|_| encde::Error::Custom("Could not extract TimeStamp from its representation"))?.encode(writer)
	}
}
impl Decode for TimeStamp {
	fn decode(reader: &mut dyn io::Read) -> encde::Result<Self> {
		let repr = Decode::decode(reader)?;
		Self::from_repr(repr).ok_or(encde::Error::Custom("Could not convert TimeStamp to its representation"))
	}
}
impl From<DateTime<Local>> for TimeStamp {
	fn from(dt: DateTime<Local>) -> Self {
		Self(dt)
	}
}
impl Default for TimeStamp {
	fn default() -> TimeStamp {
		Self::now()
	}
}

/// The V5 is a 32-bit platform.
pub type Address = u32;
pub type FileSize = u32;

pub type FileIndex = u8;
pub type PacketSize = u16;

#[derive(Encode, Decode, Clone, Copy, Eq)]
#[repr(transparent)]
pub struct FixedString<const N: usize>([u8; N]);
impl<const N: usize> Default for FixedString<N> {
	fn default() -> Self {
		Self([0u8; N])
	}
}
impl<const N: usize> std::str::FromStr for FixedString<N> {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() > N {
			return Err("String is too long");
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

pub type FileType = FixedString<4>;
// This type is the same size as String so you might as well store it by value!
pub type FileName = FixedString<24>;

/// A qualified file name, that is, one with a category.
#[derive(Debug)]
pub struct QualFileName {
	pub category: Category,
	pub name: FileName,
}
impl std::str::FromStr for QualFileName {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some((category, name)) = s.split_once(':') {
			Ok(Self {
				category: Category::from_str(category)?,
				name: FileName::from_str(name)?,
			})
		} else {
			Ok(Self {
				category: Category::default(),
				name: FileName::from_str(s)?,
			})
		}
	}
}
impl std::fmt::Display for QualFileName {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "{}:{}", self.category, self.name)
	}
}

/// A qualified file, that is, one with a category and type.
#[derive(Debug)]
pub struct QualFile {
	pub common: QualFileName,
	pub ty: FileType,
}
impl std::str::FromStr for QualFile {
	type Err = &'static str;
	// by not implementing this in terms of QualFileName::from_str, we avoid the buffering and unbuffering through FileName instances, instead using &str until we actually need to write the data.
	// &str is not actually much smaller than a FileName (16 bytes vs 24), but it allows for the easy and lightweight creation and manipulation of substrings.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut ret_category = Category::default();
		let mut ret_name = s;
		let ret_ty;

		// if there is a category, the ret_name will be the part after the colon
		// e.g., user:slot_1.ini
		if let Some((category, other)) = s.split_once(':') {
			ret_category = Category::from_str(category)?;
			ret_name = other;
		}
		// otherwise, the ret_name will be the whole string
		// e.g., slot_1.ini

		// then, using the name from the previous section, attempt to get the type
		ret_ty = if let Some((_stem, ty)) = ret_name.rsplit_once('.') { ty } else { "bin" };

		Ok(Self {
			common: QualFileName {
				category: ret_category,
				name: FileName::from_str(ret_name)?,
			},
			ty: FileType::from_str(ret_ty)?,
		})
	}
}
impl std::fmt::Display for QualFile {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "{} (type: {})", self.common, self.ty)
	}
}

#[derive(Default)]
pub struct ReadArgs {
	pub target: Target,
	pub address: Option<Address>,
	pub size: Option<FileSize>,
}

#[derive(Default)]
pub struct WriteArgs {
	pub action: TransferCompleteAction,
	pub target: Target,
	pub address: Option<Address>,
	pub overwrite: bool,
	pub timestamp: TimeStamp,
	// YAGNI
	// pub linked_filename: Option<FileName>,
	// pub linked_category: Option<Category>,
}

#[derive(Default)]
pub struct DeleteArgs {
	pub include_linked: bool,
}
