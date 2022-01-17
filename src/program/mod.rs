use crate::device::filesystem::{self as fs, QualFile};
use crate::device::{filesystem as dev_fs, Device, DeviceError as DE, ProtocolError as PE, ResponseByte, Result as DevResult};
use encde::util::VecWriter;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod slot_number;

pub use slot_number::SlotNumber;

const NUM_SLOTS: usize = 8;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramIniTopLevel {
	program: ProgramIni,
}

/// This is intentionally missing fields, or uses String where a more specific type could be used.
/// We want to keep it as simple and minimal as possible.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramIni {
	pub version: String,
	pub name: String,
	#[serde(deserialize_with = "SlotNumber::deserialize_as_index", serialize_with = "SlotNumber::serialize_as_index")]
	pub slot: SlotNumber,
	pub icon: String,
	pub description: String,
	pub date: String,
}

pub type Programs = [Option<ProgramIni>; NUM_SLOTS];

fn slot_number_to_ini_name(number: SlotNumber) -> String {
	format!("slot_{}.ini", number)
}

fn slot_number_to_bin_name(number: SlotNumber) -> String {
	format!("slot_{}.bin", number)
}

fn slot_number_to_ini_qual_file(number: SlotNumber) -> QualFile {
	QualFile::from_str(&slot_number_to_ini_name(number)).unwrap()
}

fn slot_number_to_bin_qual_file(number: SlotNumber) -> QualFile {
	QualFile::from_str(&slot_number_to_bin_name(number)).unwrap()
}

pub fn get(device: &mut Device, slot: SlotNumber) -> DevResult<Option<ProgramIni>> {
	let slot = slot_number_to_ini_qual_file(slot);
	let mut buffer = VecWriter::new();
	match device.read_file_to_stream(&mut buffer, &slot, &fs::ReadArgs::default()) {
		Ok(_) => {
			let buffer = buffer.into_inner();
			let buffer = std::str::from_utf8(&buffer).expect("Converting slot data to Unicode");
			let decoded: ProgramIniTopLevel = serde_ini::from_str(buffer).expect("Parsing slot data");
			Ok(Some(decoded.program))
		}
		Err(DE::Protocol(PE::Nack(ResponseByte::Enoent))) => Ok(None),
		Err(x) => Err(x),
	}
}

pub fn get_all(device: &mut Device) -> DevResult<Programs> {
	let mut ret: Programs = [None, None, None, None, None, None, None, None];
	for slot_num in 0..NUM_SLOTS {
		ret[slot_num as usize] = get(device, SlotNumber::from_index(slot_num).unwrap())?;
	}
	Ok(ret)
}

/// Returns whether the slot was actually removed.
pub fn remove(device: &mut Device, slot: SlotNumber, include_linked: bool) -> DevResult<bool> {
	let ini_name = slot_number_to_ini_qual_file(slot);
	let bin_name = slot_number_to_bin_qual_file(slot);
	let ini_ret = device.delete_file(&ini_name.common, &Default::default())?;
	let bin_ret = device.delete_file(&bin_name.common, &dev_fs::DeleteArgs { include_linked })?;
	Ok(ini_ret && bin_ret)
}

/// Rather than deleting everything in the user category as PROS CLI does, take a more gentle approach and only delete `slot_[1-8].(ini|bin)`.
pub fn remove_all(device: &mut Device) -> DevResult<()> {
	for slot_num in 0..NUM_SLOTS {
		let slot_num = SlotNumber::from_index(slot_num).unwrap();
		let _ = remove(device, slot_num, true)?;
	}
	Ok(())
}
