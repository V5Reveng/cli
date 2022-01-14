use crate::device::r#impl::{receive as priv_receive, send as priv_send, CommandId};
use crate::device::{filesystem, Device, DeviceError, ProtocolError, Result};
use log::debug;
use std::io::Read;

fn pad(size: filesystem::PacketSize) -> filesystem::PacketSize {
	const BITS: filesystem::PacketSize = 4 - 1;
	let base = size & !BITS;
	let extra = size & BITS;
	let extra = if extra > 0 { 4 } else { 0 };
	base + extra
}

impl Device {
	pub fn start_file_transfer(&mut self, args: &priv_send::StartFileTransfer) -> Result<priv_receive::StartFileTransfer> {
		debug!("start file transfer");
		self.ext_command_with_data(0x11, &args)
	}
	pub fn ft_read_single(&mut self, data: &mut [u8], base_address: filesystem::Address) -> Result<()> {
		const COMMAND_ID: CommandId = 0x14;
		let amount_to_read: filesystem::PacketSize = data.len().try_into().expect("Buffer is too large to read with ft_read_single");
		let amount_to_read = pad(amount_to_read);
		debug!("file transfer: rx chunk of {} (padded to {}) bytes", data.len(), amount_to_read);
		let send = priv_send::FileTransferRead { address: base_address, size: amount_to_read };
		self.begin_ext_command(COMMAND_ID, &encde::util::encode_to_vec(&send)?)?;
		let payload_len = self.rx_ext_command_header(COMMAND_ID)? - std::mem::size_of::<u32>();
		let _address = <u32 as encde::Decode>::decode(&mut self.port)?;
		if payload_len != amount_to_read as usize {
			return Err(DeviceError::Protocol(ProtocolError::BadLength {
				entity: "file transfer read packet",
				received_length: payload_len,
			}));
		}
		self.port.read_exact(data)?;
		encde::util::read_padding(&mut self.port, amount_to_read as usize - data.len())?;
		self.rx_ext_command_footer()?;
		Ok(())
	}
	/// Returns the CRC of the data that was read
	pub fn ft_read(&mut self, stream: &mut dyn std::io::Write, mut size: filesystem::FileSize, mut base_address: filesystem::Address, max_packet_size: filesystem::PacketSize) -> Result<u32> {
		debug!("file transfer: read {} bytes from 0x{:0>8x}, max packet size is {}", size, base_address, max_packet_size);
		let mut buffer = vec![0u8; max_packet_size as usize];
		let mut crc = 0;
		while size > 0 {
			let this_packet_size = std::cmp::min(size as usize, max_packet_size as usize);
			self.ft_read_single(&mut buffer[0..this_packet_size], base_address)?;
			<u32 as crate::crc::CrcComputable>::update_crc(&mut crc, &buffer[0..this_packet_size]);
			stream.write_all(&buffer[0..this_packet_size])?;
			base_address += filesystem::Address::try_from(this_packet_size).unwrap();
			size -= filesystem::FileSize::try_from(this_packet_size).unwrap();
		}
		Ok(crc)
	}
	pub fn ft_write_single(&mut self, data: &[u8], base_address: filesystem::Address) -> Result<()> {
		const COMMAND_ID: CommandId = 0x13;
		let amount_to_write: filesystem::PacketSize = data.len().try_into().expect("Buffer is too large to write with ft_write_single");
		let amount_to_write = pad(amount_to_write);
		debug!("file transfer: rx chunk of {} (padded to {}) bytes", data.len(), amount_to_write);
		self.tx_ext_command_header(COMMAND_ID, std::mem::size_of_val(&base_address) + amount_to_write as usize)?;
		self.tx(&base_address)?;
		self.tx_raw_data(data)?;
		encde::util::write_padding(&mut self.port, amount_to_write as usize - data.len())?;
		self.tx_ext_command_footer()?;
		self.end_ext_command::<()>(COMMAND_ID)?;
		Ok(())
	}
	pub fn ft_write(&mut self, stream: &mut dyn std::io::Read, mut size: filesystem::FileSize, mut base_address: filesystem::Address, max_packet_size: filesystem::PacketSize) -> Result<()> {
		debug!("file transfer: write {} to 0x{:0>8x}, max packet size is {}", size, base_address, max_packet_size);
		let mut buffer = vec![0u8; max_packet_size as usize];
		while size > 0 {
			let this_packet_size = std::cmp::min(size as usize, max_packet_size as usize);
			stream.read_exact(&mut buffer[0..this_packet_size])?;
			self.ft_write_single(&buffer[0..this_packet_size], base_address)?;
			base_address += filesystem::Address::try_from(this_packet_size).unwrap();
			size -= filesystem::FileSize::try_from(this_packet_size).unwrap();
		}
		Ok(())
	}
	pub fn ft_set_link(&mut self, linked_file: &filesystem::QualFileName) -> Result<()> {
		debug!("file transfer: set link to {}", linked_file);
		self.ext_command_with_data::<_, ()>(0x15, &priv_send::FileTransferSetLink::new(linked_file))
	}
	pub fn end_file_transfer(&mut self, action: filesystem::TransferCompleteAction) -> Result<()> {
		debug!("end file transfer");
		self.ext_command_with_data::<_, ()>(0x12, &action)
	}
}
