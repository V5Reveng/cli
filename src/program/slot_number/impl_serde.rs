use super::SlotNumber;
use serde::de::{Error as DeError, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl SlotNumber {
	pub fn deserialize_as_index<'de, D: Deserializer<'de>>(deserializer: D) -> Result<SlotNumber, D::Error> {
		let raw_value = u8::deserialize(deserializer)?;
		SlotNumber::try_from(raw_value + 1).map_err(|_| D::Error::invalid_value(Unexpected::Unsigned(raw_value as u64), &"a number from 0 to 7 inclusive"))
	}
	pub fn serialize_as_index<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_index().serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for SlotNumber {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let raw_value = u8::deserialize(deserializer)?;
		Self::try_from(raw_value).map_err(|_| D::Error::invalid_value(Unexpected::Unsigned(raw_value as u64), &"a number from 1 to 8 inclusive"))
	}
}

impl Serialize for SlotNumber {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.0.serialize(serializer)
	}
}
