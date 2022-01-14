use super::Category;

impl std::hash::Hash for Category {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		hasher.write_u8(u8::from(*self))
	}
}
