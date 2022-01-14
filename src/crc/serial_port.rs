use super::CrcComputable;
use log::{debug, trace};
use serialport::SerialPort;
use std::io::{Read, Result, Write};

pub struct CRCSerialPort {
	underlying: Box<dyn SerialPort>,
	tx_crc: u16,
	rx_crc: u16,
}

impl From<Box<dyn SerialPort>> for CRCSerialPort {
	fn from(underlying: Box<dyn SerialPort>) -> Self {
		Self { underlying, tx_crc: 0, rx_crc: 0 }
	}
}
impl CRCSerialPort {
	pub fn port(&self) -> &dyn SerialPort {
		&*self.underlying
	}
	pub fn begin_tx_crc(&mut self) {
		trace!("begin tx crc");
		self.tx_crc = 0;
	}
	pub fn begin_rx_crc(&mut self) {
		trace!("begin rx crc");
		self.rx_crc = 0;
	}
	pub fn end_rx_crc(&mut self) -> Result<bool> {
		let mut buf = [0u8; u16::BITS as usize / (u8::BITS as usize)];
		self.read_exact(&mut buf)?;
		debug!("end rx crc with checksum 0x{:02x}{:02x}", buf[0], buf[1]);
		Ok(self.rx_crc == 0)
	}
	pub fn end_tx_crc(&mut self) -> Result<()> {
		let checksum = self.tx_crc.to_be_bytes();
		debug!("end tx crc with checksum {:#04x}", self.tx_crc);
		self.write(&checksum)?;
		Ok(())
	}
}

impl Read for CRCSerialPort {
	fn read(&mut self, output: &mut [u8]) -> Result<usize> {
		let ret = self.underlying.read(output)?;
		trace!("updating rx crc with {} bytes: {:?}", output.len(), output);
		self.rx_crc.update_crc(output);
		Ok(ret)
	}
}

impl Write for CRCSerialPort {
	fn write(&mut self, slice: &[u8]) -> Result<usize> {
		trace!("updating tx crc with {} bytes: {:?}", slice.len(), slice);
		self.tx_crc.update_crc(slice);
		self.underlying.write(slice)
	}
	fn flush(&mut self) -> Result<()> {
		self.underlying.flush()
	}
}
