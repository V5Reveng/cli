use crate::device::filesystem::{FileSize, PacketSize};
use encde::Decode;

#[derive(Decode)]
pub struct StartFileTransfer {
	pub max_packet_size: PacketSize,
	pub file_size: FileSize,
	pub crc: u32,
}
