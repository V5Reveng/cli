use super::Category;
use std::hash::{Hash, Hasher};

impl Hash for Category {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		hasher.write_u8(u8::from(*self))
	}
}
