use crate::device::Device;

#[derive(Debug)]
pub enum Presence {
	None,
	One(Device),
	Many(Vec<Device>),
}

impl From<Vec<Device>> for Presence {
	fn from(mut items: Vec<Device>) -> Self {
		match items.len() {
			0 => Self::None,
			1 => Self::One(items.pop().unwrap()),
			_ => Self::Many(items),
		}
	}
}

impl From<Option<Device>> for Presence {
	fn from(opt: Option<Device>) -> Self {
		match opt {
			None => Self::None,
			Some(x) => Self::One(x),
		}
	}
}

#[derive(Debug)]
pub enum NotOne {
	None,
	Many,
}

impl std::fmt::Display for NotOne {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let s = match self {
			Self::None => "No uploadable devices were found.",
			Self::Many => "Multiple uploadable devices were found. You can list devices with the `device list` command, and specify the device with `--device`.",
		};
		formatter.write_str(s)
	}
}
impl std::error::Error for NotOne {}

impl Presence {
	pub fn as_result(self) -> Result<Device, NotOne> {
		match self {
			Self::None => Err(NotOne::None),
			Self::One(item) => Ok(item),
			Self::Many(_) => Err(NotOne::Many),
		}
	}
}
