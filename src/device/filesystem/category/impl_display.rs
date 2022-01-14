use super::Category;

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
