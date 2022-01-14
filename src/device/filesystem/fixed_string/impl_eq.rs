use super::FixedString;

impl<const N: usize> PartialEq for FixedString<N> {
	fn eq(&self, other: &Self) -> bool {
		for (&c1, &c2) in self.0.iter().zip(other.0.iter()) {
			if c1 != c2 {
				return false;
			}
			// end of string: quit early
			// c1 == c2 due to previous block
			if c1 == 0 {
				return true;
			}
		}
		true
	}
}
