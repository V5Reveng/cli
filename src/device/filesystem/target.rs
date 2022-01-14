use encde::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Target {
	Ddr = 0,
	Flash = 1,
	Screen = 2,
}
impl Default for Target {
	fn default() -> Self {
		Self::Flash
	}
}
