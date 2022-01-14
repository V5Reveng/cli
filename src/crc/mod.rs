const TABLE_SIZE: usize = u8::MAX as usize + 1;

pub trait CrcComputable: Copy {
	const TABLE: [Self; TABLE_SIZE];
	fn update_crc(&mut self, data: &[u8]) -> &mut Self;
}

pub mod for_u16;
pub mod for_u32;
pub mod serial_port;

pub use serial_port::CRCSerialPort;
