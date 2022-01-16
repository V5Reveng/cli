use super::FixedString;
use std::hash::{Hash, Hasher};

impl<const N: usize> Hash for FixedString<N> {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.as_bytes().hash(hasher)
	}
}
