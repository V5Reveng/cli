use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use encde::{Decode, Encode, Error as EError};
use std::fmt::{self, Display, Formatter};
use std::io::{Read, Write};
use std::num::TryFromIntError;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct TimeStamp(DateTime<Local>);

impl TimeStamp {
	pub fn now() -> Self {
		Self(Local::now())
	}
	fn as_repr(&self) -> Result<u32, TryFromIntError> {
		let base_time = Local.ymd(2000, 1, 1).and_hms(0, 0, 0);
		(self.0 - base_time).num_seconds().try_into()
	}
	fn from_repr(repr: u32) -> Option<Self> {
		let base_time = NaiveDateTime::new(NaiveDate::from_ymd(2000, 1, 1), NaiveTime::from_hms(0, 0, 0));
		let base_time = Local.from_local_datetime(&base_time).single()?;
		Some(Self(base_time + Duration::seconds(repr as i64)))
	}
}

impl Display for TimeStamp {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		self.0.fmt(formatter)
	}
}

impl Encode for TimeStamp {
	fn encode(&self, writer: &mut dyn Write) -> encde::Result<()> {
		self.as_repr().map_err(|_| EError::Custom("Could not extract TimeStamp from its representation"))?.encode(writer)
	}
}

impl Decode for TimeStamp {
	fn decode(reader: &mut dyn Read) -> encde::Result<Self> {
		let repr = Decode::decode(reader)?;
		Self::from_repr(repr).ok_or(EError::Custom("Could not convert TimeStamp to its representation"))
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
