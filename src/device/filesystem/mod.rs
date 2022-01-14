use encde::{Decode, Encode};

pub mod args;
pub mod category;
pub mod channel;
pub mod fixed_string;
pub mod function;
pub mod qual;
pub mod target;
pub mod timestamp;

pub use args::*;
pub use category::*;
pub use channel::*;
pub use fixed_string::*;
pub use function::*;
pub use qual::*;
pub use target::*;
pub use timestamp::*;

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransferCompleteAction {
	NoRun = 0b00,
	RunImmediately = 0b01,
	RunScreen = 0b11,
}
impl Default for TransferCompleteAction {
	fn default() -> Self {
		Self::NoRun
	}
}

/// The V5 is a 32-bit platform.
pub type Address = u32;

pub type FileSize = u32;

pub type FileIndex = u8;

pub type PacketSize = u16;

pub type FileType = FixedString<4>;

// This type is the same size as String so you might as well store it by value!
pub type FileName = FixedString<24>;
