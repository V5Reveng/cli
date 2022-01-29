//! Trivial associated methods that make other methods slightly easier to implement.

use super::super::{Device, Result};
use core::any::type_name;
use encde::{Decode, Encode};
use log::trace;
use std::io::{Read, Write};

impl Device {
	/// Receive an arbitrary `Decode`able entity.
	pub fn rx<T: Decode + std::fmt::Debug>(&mut self) -> Result<T> {
		let ret = Decode::decode(&mut self.port)?;
		trace!("rx {}: {:?}", type_name::<T>(), ret);
		Ok(ret)
	}
	/// Transmit an arbitrary `Encode`able entity.
	pub fn tx<T: Encode + std::fmt::Debug>(&mut self, data: &T) -> Result<()> {
		trace!("tx {}: {:?}", type_name::<T>(), data);
		Encode::encode(data, &mut self.port)?;
		Ok(())
	}
	/// Receive raw data without having to use `std::io::Read`.
	pub fn rx_raw_data<const N: usize>(&mut self) -> Result<[u8; N]> {
		let mut output: [u8; N] = [0; N];
		self.port.read_exact(&mut output)?;
		trace!("rx {}: {:?}", type_name::<[u8; N]>(), output);
		Ok(output)
	}
	/// Transmit raw data without having to use `std::io::Write`.
	pub fn tx_raw_data(&mut self, data: &[u8]) -> Result<()> {
		trace!("tx {}: {:?}", type_name::<&[u8]>(), data);
		self.port.write_all(data)?;
		Ok(())
	}
}
