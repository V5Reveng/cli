use encde::{Decode, Encode};

/// The target of file transfers.
#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Target {
	Ddr = 0,
	Flash = 1,
	/// Download only
	Screen = 2,
}

impl Default for Target {
	fn default() -> Self {
		Self::Flash
	}
}
