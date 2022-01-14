use super::Category;
use std::fmt::{self, Display, Formatter};

impl Display for Category {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
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
