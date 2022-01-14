pub mod error;
pub mod impl_display;
pub mod impl_encde;
pub mod impl_eq;
pub mod impl_from_str;
pub mod impl_hash;
pub mod impl_u8_conv;

pub use error::CategoryFromStrError;

#[derive(Debug, Eq, Clone, Copy)]
pub enum Category {
	None,
	User,
	System,
	Rms,
	Pros,
	Mw,
	Unnamed(u8),
}

impl Default for Category {
	fn default() -> Self {
		Self::User
	}
}
