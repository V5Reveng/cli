use encde::{Decode, Encode};

#[derive(Debug, Eq, Clone, Copy)]
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
#[derive(Debug)]
pub enum CategoryFromStrError {
	UnknownCategory,
}
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
impl std::fmt::Display for CategoryFromStrError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let s = match self {
			Self::UnknownCategory => "unknown category",
		};
		formatter.write_str(s)
	}
}
impl std::error::Error for CategoryFromStrError {}
impl std::hash::Hash for Category {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		hasher.write_u8(u8::from(*self))
	}
}
impl PartialEq for Category {
	fn eq(&self, other: &Self) -> bool {
		u8::from(*self) == u8::from(*other)
	}
}
