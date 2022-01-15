use encde::{Decode, Encode};

/// The channel that the device is functioning on.
///
/// PROS switches to the download channel for ostensibly faster downloads, but presumably it restricts functionality.
#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
	Pit = 0,
	Download = 1,
}

impl Default for Channel {
	fn default() -> Self {
		Self::Pit
	}
}
