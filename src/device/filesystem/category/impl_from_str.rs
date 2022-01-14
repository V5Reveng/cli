use super::{Category, CategoryFromStrError};

impl std::str::FromStr for Category {
	type Err = CategoryFromStrError;
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
				Err(_) => Err(Self::Err::UnknownCategory),
			},
		}
	}
}
