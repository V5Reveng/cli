use crate::commands::Runnable;
use std::io::stdout;
use v5_device::device::filesystem as fs;
use v5_device::device::{DeviceError, ProtocolError, ResponseByte};

/// Output the contents of a file.
///
/// To "pull" a file from the device, you can add ` > local.file` to the command line.
#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: fs::QualFile,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		let contents = dev.read_file_to_stream(&mut stdout(), &self.file, &fs::ReadArgs { ..Default::default() });
		match contents {
			Err(DeviceError::Protocol(ProtocolError::Nack(ResponseByte::ProgramFileError))) => {
				anyhow::bail!("File does not exist");
			}
			x => Ok(x?),
		}
	}
}
