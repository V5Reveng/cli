use super::Category;
use std::fmt::{self, Display, Formatter};

impl Display for Category {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		let name = match *self {
			Self::NONE => "(none)",
			Self::USER => "user",
			Self::SYSTEM => "system",
			Self::RMS => "rms",
			Self::PROS => "pros",
			Self::MW => "mw",
			x => {
				return write!(formatter, "0x{:02x}", x.0);
			}
		};
		formatter.write_str(name)
	}
}
