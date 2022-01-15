pub mod error;
pub mod impl_display;
pub mod impl_encde;
pub mod impl_eq;
pub mod impl_from_str;
pub mod impl_hash;
pub mod impl_u8_conv;

pub use error::CategoryFromStrError;

/// The category of a file.
///
/// This is a primitive way to create subdirectories on the device's filesystem.
/// Each category acts as one subdirectory of the root directory.
#[derive(Debug, Eq, Clone, Copy)]
pub enum Category {
	/// Only received, never sent.
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
