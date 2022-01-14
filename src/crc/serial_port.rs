use super::CrcComputable;
use log::{debug, trace};
use serialport::SerialPort;

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
	pub fn end_rx_crc(&mut self) -> std::io::Result<bool> {
		let mut buf = [0u8; u16::BITS as usize / (u8::BITS as usize)];
		<Self as std::io::Read>::read_exact(self, &mut buf)?;
		debug!("end rx crc with checksum 0x{:02x}{:02x}", buf[0], buf[1]);
		Ok(self.rx_crc == 0)
	}
	pub fn end_tx_crc(&mut self) -> std::io::Result<()> {
		let checksum = self.tx_crc.to_be_bytes();
		debug!("end tx crc with checksum {:#04x}", self.tx_crc);
		<Self as std::io::Write>::write(self, &checksum)?;
		Ok(())
	}
}

impl std::io::Read for CRCSerialPort {
	fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
		let ret = self.underlying.read(output)?;
		trace!("updating rx crc with {} bytes: {:?}", output.len(), output);
		self.rx_crc.update_crc(output);
		Ok(ret)
	}
}

impl std::io::Write for CRCSerialPort {
	fn write(&mut self, slice: &[u8]) -> std::io::Result<usize> {
		trace!("updating tx crc with {} bytes: {:?}", slice.len(), slice);
		self.tx_crc.update_crc(slice);
		self.underlying.write(slice)
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.underlying.flush()
	}
}
