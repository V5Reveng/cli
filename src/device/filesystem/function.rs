use encde::{Decode, Encode};

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Function {
	Upload = 1,
	Download = 2,
}
