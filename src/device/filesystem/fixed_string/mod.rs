//! Fixed-size strings stored inline.

use encde::{Decode, Encode};

pub mod error;
pub mod impl_eq;
pub mod impl_format;
pub mod impl_from_str;
pub mod impl_hash;

pub use error::FixedStringFromStrError;

#[derive(Encode, Decode, Clone, Copy, Eq)]
#[repr(transparent)]
pub struct FixedString<const N: usize>([u8; N]);
impl<const N: usize> Default for FixedString<N> {
	fn default() -> Self {
		Self([0u8; N])
	}
}

impl<const N: usize> FixedString<N> {
	pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
		let mut len = N;
		for (idx, &byte) in self.0.iter().enumerate() {
			if byte == 0 {
				len = idx;
				break;
			}
		}
		std::str::from_utf8(&self.0[..len])
	}
}
