use super::Category;
use encde::{Decode, Encode};
use std::io::{Read, Write};

impl Encode for Category {
	fn encode(&self, writer: &mut dyn Write) -> encde::Result<()> {
		let value = u8::from(*self);
		value.encode(writer)
	}
}

impl Decode for Category {
	fn decode(reader: &mut dyn Read) -> encde::Result<Self> {
		let value = u8::decode(reader)?;
		Ok(Self::from(value))
	}
}
