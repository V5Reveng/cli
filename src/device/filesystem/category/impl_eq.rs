use super::Category;

impl PartialEq for Category {
	fn eq(&self, other: &Self) -> bool {
		u8::from(*self) == u8::from(*other)
	}
}
