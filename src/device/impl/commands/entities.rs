//! The entities involved in sending commands.

use crate::device::r#impl::CommandId;
use crate::device::{Device, Result};
use encde::{util::encode_to_vec, Decode, Encode};

impl Device {
	/// Send a simple command without sending a payload.
	pub fn simple_command_no_data<T: Decode>(&mut self, command_id: CommandId) -> Result<T> {
		self.begin_simple_command(command_id)?;
		self.end_simple_command(command_id)
	}
	/// Send an extended command without sending a payload.
	pub fn ext_command_no_data<T: Decode>(&mut self, command_id: CommandId) -> Result<T> {
		self.begin_ext_command(command_id, &[])?;
		self.end_ext_command(command_id)
	}
	/// Send an extended command including a sent payload.
	pub fn ext_command_with_data<S: Encode, R: Decode>(&mut self, command_id: CommandId, send_data: &S) -> Result<R> {
		let encoded = encode_to_vec(send_data)?;
		self.begin_ext_command(command_id, &encoded)?;
		self.end_ext_command(command_id)
	}
}
