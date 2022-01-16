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

/// What to do when the file transfer, specifically of an executable, completes.
#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// (The V5 is a 32-bit platform.)
pub type Address = u32;
/// The default address for file uploads.
pub const DEFAULT_ADDRESS: Address = 0x07_80_00_00;

pub type FileSize = u32;

/// This implies that there is a maximum of 256 files in one category.
pub type FileIndex = u8;

pub type PacketSize = u16;

/// Max 4 chars. In "slot_1.bin" it would be "bin".
pub type FileType = FixedString<4>;

/// Includes the extension. Max 24 chars.
///
// This type is the same size as String so you might as well store it by value!
pub type FileName = FixedString<24>;
