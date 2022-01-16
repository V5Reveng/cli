use super::{Category, CategoryFromStrError};
use std::str::FromStr;

impl FromStr for Category {
	type Err = CategoryFromStrError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"default" => Ok(Self::default()),
			"user" => Ok(Self::USER),
			"system" => Ok(Self::SYSTEM),
			"rms" => Ok(Self::RMS),
			"pros" => Ok(Self::PROS),
			"mw" => Ok(Self::MW),
			numeric => Ok(Self(crate::util::num::lenient_u64_from_str(numeric).map_err(|_| Self::Err::UnknownCategory)?.try_into().map_err(|_| Self::Err::TooLarge)?)),
		}
	}
}
