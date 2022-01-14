use crate::device::r#impl::CommandId;
use crate::device::{Device, DeviceError, ProtocolError, ResponseByte, Result};
use log::{debug, trace};

impl Device {
	pub fn begin_simple_command(&mut self, command_id: CommandId) -> Result<()> {
		debug!("begin simple command {:#02x}", command_id);
		self.tx_command_header()?;
		self.tx(&command_id)?;
		Ok(())
	}
	pub fn end_simple_command<T: encde::Decode>(&mut self, command_id: CommandId) -> Result<T> {
		debug!("end simple command {:#02x}", command_id);
		self.rx_response_header()?;
		self.rx_echoed_command(command_id)?;
		self.rx_simple_payload()
	}
	pub fn tx_ext_command_header(&mut self, command_id: CommandId, payload_len: usize) -> Result<()> {
		debug!("begin extended command {:#02x} with {} bytes of data", command_id, payload_len);
		self.port.begin_tx_crc();
		self.tx_command_header()?;
		self.tx(&Self::EXT_COMMAND)?;
		self.tx(&command_id)?;
		self.tx_vex_varint(payload_len)?;
		Ok(())
	}
	pub fn tx_ext_command_footer(&mut self) -> Result<()> {
		self.port.end_tx_crc().map_err(DeviceError::from)
	}
	pub fn begin_ext_command(&mut self, command_id: CommandId, data: &[u8]) -> Result<()> {
		self.tx_ext_command_header(command_id, data.len())?;
		self.tx_raw_data(data)?;
		self.tx_ext_command_footer()?;
		Ok(())
	}
	pub fn rx_ext_command_header(&mut self, sent_command: CommandId) -> Result<usize> {
		trace!("rx extended command header");
		self.port.begin_rx_crc();
		self.rx_response_header()?;
		self.rx_expect("echoed command", &Self::EXT_COMMAND)?;
		// subtract echoed command byte, plus 16-bit CRC in rx_ext_command_footer
		let payload_len = self.rx_ext_payload_length()? - 3;
		self.rx_expect("echoed actual command", &sent_command)?;
		Ok(payload_len)
	}
	pub fn rx_ext_command_footer(&mut self) -> Result<()> {
		trace!("rx extended command footer");
		if self.port.end_rx_crc()? {
			Ok(())
		} else {
			Err(DeviceError::Protocol(ProtocolError::InvalidCrc))
		}
	}
	pub fn end_ext_command<T: encde::Decode>(&mut self, sent_command: CommandId) -> Result<T> {
		debug!("end extended command {:#02x}", sent_command);
		// subtract response byte
		let payload_len = self.rx_ext_command_header(sent_command)? - 1;
		let response_byte = self.rx_response_byte()?;
		let raw_payload = self.rx_bytes(payload_len)?;
		self.rx_ext_command_footer()?;
		if response_byte == ResponseByte::Ack {
			Self::decode_from_data(&raw_payload)
		} else {
			Err(ProtocolError::Nack(response_byte).into())
		}
	}
}
