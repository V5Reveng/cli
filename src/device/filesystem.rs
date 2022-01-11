use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use encde::{Decode, Encode};
use std::io;

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Function {
	Upload = 1,
	Download = 2,
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Category {
	None = 0,
	User = 1,
	System = 15,
	RMS = 16,
	PROS = 24,
	MW = 32,
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
			Self::RMS => "RobotMesh Studio",
			Self::PROS => "PROS",
			Self::MW => "MW",
		};
		formatter.write_str(name)
	}
}

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
	PIT = 0,
	Download = 1,
}

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Target {
	DDR = 0,
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
	fn to_repr(&self) -> Result<u32, std::num::TryFromIntError> {
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
		self.to_repr().map_err(|_| encde::Error::Custom("Could not extract TimeStamp from its representation"))?.encode(writer)
	}
}
impl Decode for TimeStamp {
	fn decode(reader: &mut dyn io::Read) -> encde::Result<Self> {
		let repr = Decode::decode(reader)?;
		Ok(Self::from_repr(repr).ok_or(encde::Error::Custom("Could not convert TimeStamp to its representation"))?)
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

macro_rules! fixed_string_type {
	($ident:ident, $size:tt) => {
		#[derive(Encode, Decode, Default, Clone, Copy, Eq)]
		#[repr(transparent)]
		pub struct $ident([u8; $size]);
		impl std::str::FromStr for $ident {
			type Err = ();
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				if s.len() > $size {
					return Err(());
				}
				let mut ret: Self = Self::default();
				for (idx, byte) in s.bytes().chain(std::iter::repeat(0)).take($size).enumerate() {
					ret.0[idx] = byte;
				}
				Ok(ret)
			}
		}
		impl $ident {
			pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
				let mut len = $size;
				for (idx, &byte) in self.0.iter().enumerate() {
					if byte == 0 {
						len = idx;
						break;
					}
				}
				std::str::from_utf8(&self.0[..len])
			}
		}
		impl std::fmt::Debug for $ident {
			fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				use std::fmt::Write;
				formatter.write_str(stringify!($ident))?;
				formatter.write_char(' ')?;
				self.as_str().map_err(|_| std::fmt::Error::default())?.fmt(formatter)
			}
		}
		impl std::fmt::Display for $ident {
			fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.as_str().map_err(|_| std::fmt::Error::default())?.fmt(formatter)
			}
		}
	};
}

fixed_string_type!(FileType, 4);
// This type is the same size as String so you might as well store it by value!
fixed_string_type!(FileName, 24);

#[derive(Default)]
pub struct ReadArgs {
	pub file_name: FileName,
	pub file_type: FileType,
	pub category: Category,
	pub target: Target,
	pub address: Option<Address>,
	pub size: Option<FileSize>,
}

#[derive(Default)]
pub struct WriteArgs {
	pub file_name: FileName,
	pub file_type: FileType,
	pub action: TransferCompleteAction,
	pub category: Category,
	pub target: Target,
	pub address: Option<Address>,
	pub overwrite: bool,
	pub timestamp: TimeStamp,
	// YAGNI
	// pub linked_filename: Option<FileName>,
	// pub linked_category: Option<Category>,
}
