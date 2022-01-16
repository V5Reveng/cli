use encde::{Decode, Encode};

/// The channel that the device is functioning on.
///
/// PROS CLI calls the file transfer channel the download channel.
#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Channel {
	Pit = 0,
	FileTransfer = 1,
}

impl Default for Channel {
	fn default() -> Self {
		Self::Pit
	}
}
