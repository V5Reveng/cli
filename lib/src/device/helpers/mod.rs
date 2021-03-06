use encde::Decode;
use std::fmt::{self, Display, Formatter};

pub mod version;
pub use version::{LongVersion, ShortVersion};

#[derive(Decode, Clone, Copy)]
pub struct BrainFlags(u8);
impl BrainFlags {
	// empty
}

#[derive(Decode, Clone, Copy)]
pub struct ControllerFlags(u8);
impl ControllerFlags {
	pub fn connected(&self) -> bool {
		self.0 & 0b1 == 0b1
	}
	pub fn wireless(&self) -> bool {
		self.0 & 0b10 == 0b10
	}
}

#[derive(Decode, Clone, Copy)]
#[repr(u8)]
pub enum Product {
	#[encde(wire_tag = 0x10)]
	Brain(BrainFlags),
	#[encde(wire_tag = 0x11)]
	Controller(ControllerFlags),
}

impl Display for Product {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Brain(_) => f.write_str("brain"),
			Self::Controller(flags) => write!(f, "controller (connected: {}; wireless: {})", flags.connected(), flags.wireless()),
		}
	}
}

pub type SystemId = u32;
