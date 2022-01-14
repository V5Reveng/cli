use serialport::Error;

#[derive(Debug)]
pub enum UploadableInfoFromPathError {
	/// Path is not valid UTF-8
	PathNotUTF8,
	/// Path does not exist
	Nonexistent,
	/// Error in the underlying serialport library
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
