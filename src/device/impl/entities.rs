use crate::device::r#impl::CommandId;
use crate::device::{Device, DeviceError, ProtocolError, ResponseByte, Result};
use encde::util::{decode_from_entire_slice, encode_to_vec};
use encde::{Decode, Encode};
use log::{debug, trace};
use std::io::Read;

impl Device {
	pub const EXT_COMMAND: CommandId = 0x56;

	pub fn tx_command_header(&mut self) -> Result<()> {
		debug!("tx command header");
		const HEADER: [u8; 4] = [0xc9, 0x36, 0xb8, 0x47];
		self.tx_raw_data(&HEADER)?;
		Ok(())
	}
	pub fn rx_response_header(&mut self) -> Result<()> {
		debug!("rx response header");
		let buf: [u8; 2] = self.rx_raw_data()?;
		const EXPECTED: [u8; 2] = [0xaa, 0x55];
		if buf != EXPECTED {
			Err(DeviceError::Protocol(ProtocolError::WrongData {
				entity: "Response header",
				expected: EXPECTED.into(),
				received: buf.into(),
			}))
		} else {
			Ok(())
		}
	}
	pub fn rx_echoed_command(&mut self, expected: CommandId) -> Result<()> {
		debug!("rx echoed command {:02x}", expected);
		let echoed_command: u8 = self.rx()?;
		if echoed_command != expected {
			Err(DeviceError::Protocol(ProtocolError::WrongData {
				entity: "Echoed command",
				expected: [expected].into(),
				received: [echoed_command].into(),
			}))
		} else {
			Ok(())
		}
	}
	pub fn rx_simple_payload_length(&mut self) -> Result<usize> {
		debug!("rx simple payload length");
		Ok(self.rx::<u8>()? as usize)
	}
	pub fn rx_bytes(&mut self, payload_length: usize) -> Result<Vec<u8>> {
		trace!("rx {} bytes", payload_length);
		let mut payload: Vec<u8> = vec![0; payload_length];
		self.port.read_exact(payload.as_mut_slice())?;
		Ok(payload)
	}
	pub fn rx_simple_raw_payload(&mut self) -> Result<Vec<u8>> {
		debug!("rx simple raw payload");
		let payload_length = self.rx_simple_payload_length()?;
		self.rx_bytes(payload_length)
	}
	pub fn decode_from_data<T: Decode>(data: &[u8]) -> Result<T> {
		Ok(decode_from_entire_slice(data)?)
	}
	pub fn rx_simple_payload<T: Decode>(&mut self) -> Result<T> {
		debug!("rx simple payload");
		let raw = self.rx_simple_raw_payload()?;
		Self::decode_from_data(&raw)
	}
	pub fn tx_vex_varint(&mut self, length: usize) -> Result<()> {
		debug!("tx vex variable-length int: {}", length);
		match length {
			0..=0x7f => self.tx(&(length as u8)),
			0x80..=0x7fff => self.tx(&[((length >> 8) | 0x80) as u8, (length & 0xff) as u8]),
			actual => Err(DeviceError::Protocol(ProtocolError::OutOfRange {
				entity: "variable-length integer",
				min: 0,
				max: 0x7fff,
				actual,
			})),
		}
	}
	pub fn rx_vex_varint(&mut self) -> Result<usize> {
		let mut ret = self.rx::<u8>()? as usize;
		if ret & 0x80 == 0x80 {
			ret &= 0x7f;
			ret <<= 8;
			ret += self.rx::<u8>()? as usize;
		}
		debug!("rx vex variable-length int: {}", ret);
		Ok(ret)
	}
	pub fn rx_ext_payload_length(&mut self) -> Result<usize> {
		let ret = self.rx_vex_varint()?;
		debug!("rx extended payload length");
		Ok(ret)
	}
	pub fn rx_expect<T: Decode + Encode + PartialEq + std::fmt::Debug>(&mut self, entity: &'static str, expected: &T) -> Result<()> {
		let received = self.rx::<T>()?;
		if &received != expected {
			Err(DeviceError::Protocol(ProtocolError::WrongData {
				entity,
				expected: encode_to_vec(expected)?.into(),
				received: encode_to_vec(&received)?.into(),
			}))
		} else {
			trace!("rx {}, expecting value {:?}", entity, expected);
			Ok(())
		}
	}
	pub fn rx_response_byte(&mut self) -> Result<ResponseByte> {
		debug!("rx response byte");
		self.rx()
	}
}
