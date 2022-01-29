use crate::device::{Device, DeviceError, Result};
use std::time::Duration;

impl Device {
	pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

	pub fn timeout(&self) -> Duration {
		self.port.port().timeout()
	}
	pub fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
		self.port.port_mut().set_timeout(timeout).map_err(DeviceError::from)
	}
	pub fn reset_timeout(&mut self) -> Result<()> {
		self.set_timeout(Self::DEFAULT_TIMEOUT)
	}
}
