use super::Category;

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
