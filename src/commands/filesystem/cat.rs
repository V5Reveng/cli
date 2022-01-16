use crate::commands::Runnable;
use crate::device::filesystem as fs;
use crate::device::{DeviceError, ProtocolError, ResponseByte};
use log::error;
use std::io::stdout;

/// Output the contents of a file.
///
/// To "pull" a file from the device, you can add ` > local.file` to the command line.
#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: fs::QualFile,
}

impl Runnable for Args {
	fn run(self, dev: crate::util::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let contents = dev.read_file_to_stream(&mut stdout(), &self.file, &fs::ReadArgs { ..Default::default() });
		match contents {
			Err(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::ProgramFileError))) => {
				error!("File does not exist: {}", self.file);
				1
			}
			x => {
				x.unwrap();
				0
			}
		}
	}
}
