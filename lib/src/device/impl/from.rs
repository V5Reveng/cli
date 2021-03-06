use crate::device::discover::UploadableInfo;
use crate::device::Device;
use log::debug;
use std::path::Path;

const SERIAL_BAUD: u32 = 115200;

impl<'a> TryFrom<&'a Path> for Device {
	type Error = <UploadableInfo as TryFrom<&'a Path>>::Error;
	fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
		UploadableInfo::try_from(path).and_then(|info| Device::try_from(info).map_err(Self::Error::SerialPortError))
	}
}

impl TryFrom<UploadableInfo> for Device {
	type Error = serialport::Error;
	fn try_from(info: UploadableInfo) -> Result<Self, Self::Error> {
		use serialport::*;
		debug!("Opening serial port {} for V5 device of type {:?}", &info.name, &info.device_type);
		Ok(Device {
			ty: info.device_type,
			port: serialport::new(info.name, SERIAL_BAUD)
				.parity(Parity::None)
				.stop_bits(StopBits::One)
				.data_bits(DataBits::Eight)
				.flow_control(FlowControl::None)
				.timeout(Self::DEFAULT_TIMEOUT)
				.open()?
				.into(),
		})
	}
}
