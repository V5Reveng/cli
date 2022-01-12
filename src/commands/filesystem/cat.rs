use crate::commands::Runnable;
use crate::device::filesystem as fs;
use log::error;
use std::io::stdout;

#[derive(clap::Parser)]
pub struct Args {
	file: fs::QualFile,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let contents = dev.read_file_to_stream(&mut stdout(), &self.file, &crate::device::filesystem::ReadArgs { ..Default::default() });
		match contents {
			Err(crate::device::DeviceError::Protocol(crate::device::ProtocolError::Nack(crate::device::ResponseByte::ProgramFileError))) => {
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
