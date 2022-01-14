use encde::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
	Pit = 0,
	Download = 1,
}
