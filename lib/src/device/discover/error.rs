use serialport::Error;

#[derive(Debug)]
pub enum UploadableInfoFromPathError {
	PathNotUtf8,
	Nonexistent,
	SerialPortError(Error),
	/// The path, in some way, does not refer to a valid uploadable device. Among others:
	/// - The path does exist, but is not a serial port.
	/// - The path is a serial port, but not an uploadable VEX device as identified by the platform-specific implementation.
	NotValid,
}

impl From<Error> for UploadableInfoFromPathError {
	fn from(e: Error) -> UploadableInfoFromPathError {
		UploadableInfoFromPathError::SerialPortError(e)
	}
}

impl std::fmt::Display for UploadableInfoFromPathError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::PathNotUtf8 => write!(formatter, "Path is not UTF-8"),
			Self::Nonexistent => write!(formatter, "Path does not exist"),
			Self::SerialPortError(underlying) => write!(formatter, "Serial port error: {}", underlying),
			Self::NotValid => write!(formatter, "Path does not refer to a valid Uploadable"),
		}
	}
}
impl std::error::Error for UploadableInfoFromPathError {}
