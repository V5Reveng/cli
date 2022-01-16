//! Payloads to be received with commands. These are private to the `Device`, as opposed to `super::receive` which is public to the crate.

use crate::device::filesystem::{FileSize, PacketSize};
use encde::Decode;

#[derive(Decode)]
pub struct StartFileTransfer {
	/// The maximum packet size for following read or write commands.
	pub max_packet_size: PacketSize,
	/// Download only
	pub file_size: FileSize,
	/// Download only
	pub crc: u32,
}
