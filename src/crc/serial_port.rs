use super::CrcComputable;
use log::{debug, trace};
use serialport::SerialPort;
use std::io::{Read, Result, Write};

/// A serial port that passively calculates the 16-bit CRC of the data that passes through it in both directions.
pub struct CrcSerialPort {
	underlying: Box<dyn SerialPort>,
	tx_crc: u16,
	rx_crc: u16,
}

impl From<Box<dyn SerialPort>> for CrcSerialPort {
	fn from(underlying: Box<dyn SerialPort>) -> Self {
		Self { underlying, tx_crc: 0, rx_crc: 0 }
	}
}

impl CrcSerialPort {
	pub fn port(&self) -> &dyn SerialPort {
		&*self.underlying
	}
	/// Start calculating the CRC for sent data.
	/// Use should be paired with `end_tx_crc`.
	pub fn begin_tx_crc(&mut self) {
		trace!("begin tx crc");
		self.tx_crc = 0;
	}
	/// Start calculating the CRC for received data.
	/// Use should be paired with `end_rx_crc`.
	pub fn begin_rx_crc(&mut self) {
		trace!("begin rx crc");
		self.rx_crc = 0;
	}
	/// Read a CRC checksum and calculate it.
	/// Returns whether the data received since the last call to `begin_rx_crc` passed the CRC check.
	pub fn end_rx_crc(&mut self) -> Result<bool> {
		let mut buf = [0u8; u16::BITS as usize / (u8::BITS as usize)];
		self.read_exact(&mut buf)?;
		debug!("end rx crc with checksum 0x{:02x}{:02x}", buf[0], buf[1]);
		Ok(self.rx_crc == 0)
	}
	/// Writes the CRC checksum since the last call to `begin_tx_crc`.
	pub fn end_tx_crc(&mut self) -> Result<()> {
		let checksum = self.tx_crc.to_be_bytes();
		debug!("end tx crc with checksum {:#04x}", self.tx_crc);
		self.write_all(&checksum)?;
		Ok(())
	}
}

/// Reading reads from the underlying port, calculating the CRC before returning as usual.
impl Read for CrcSerialPort {
	fn read(&mut self, output: &mut [u8]) -> Result<usize> {
		let ret = self.underlying.read(output)?;
		trace!("updating rx crc with {} bytes: {:?}", ret, output);
		self.rx_crc.update_crc(&output[0..ret]);
		Ok(ret)
	}
}

/// Writing writes to the underlying port, calculating the CRC of the data that was written before returning as usual.
impl Write for CrcSerialPort {
	fn write(&mut self, slice: &[u8]) -> Result<usize> {
		trace!("updating tx crc with up to {} bytes: {:?}", slice.len(), slice);
		let ret = self.underlying.write(slice)?;
		self.tx_crc.update_crc(&slice[0..ret]);
		trace!("updated tx crc with {} of {} bytes (rest were not written)", ret, slice.len());
		Ok(ret)
	}
	fn flush(&mut self) -> Result<()> {
		self.underlying.flush()
	}
}
