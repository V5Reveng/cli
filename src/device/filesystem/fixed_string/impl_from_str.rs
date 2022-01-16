use super::{FixedString, FixedStringFromStrError as Error};
use std::convert::TryFrom;

impl<const N: usize> TryFrom<&[u8]> for FixedString<N> {
	type Error = Error;
	fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
		if data.len() > N {
			return Err(Self::Error::TooLong);
		}
		if let Some(nul_pos) = data.iter().position(|&x| x == 0) {
			return Err(Self::Error::ContainsNul { position: nul_pos });
		}
		let mut ret: Self = Self::default();
		// default is zeroed so no need to zero-fill again; just write the actual data.
		for (idx, &byte) in data.iter().enumerate() {
			ret.0[idx] = byte;
		}
		Ok(ret)
	}
}
