use super::FixedString;
use std::hash::{Hash, Hasher};

impl<const N: usize> Hash for FixedString<N> {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		for c in self.0 {
			if c == 0 {
				break;
			}
			hasher.write_u8(c);
		}
	}
}
