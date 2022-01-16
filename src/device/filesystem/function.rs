use encde::{Decode, Encode};

/// The functions available for a file transfer.
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Function {
	/// Write a file to the device.
	Upload = 1,
	/// Read a file from the device.
	Download = 2,
}
