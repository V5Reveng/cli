use super::FixedString;

impl<const N: usize> std::hash::Hash for FixedString<N> {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		for c in self.0 {
			if c == 0 {
				break;
			}
			hasher.write_u8(c);
		}
	}
}
