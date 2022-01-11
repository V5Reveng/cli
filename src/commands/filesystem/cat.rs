use crate::commands::Runnable;
use log::error;
use std::io::stdout;

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: String,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let (file_name, file_type) = crate::commands::string_to_file_name_and_type(&self.file);
		let contents = dev.read_file_to_stream(
			&mut stdout(),
			&crate::device::filesystem::ReadArgs {
				file_name,
				file_type,
				..Default::default()
			},
		);
		let _ = match contents {
			Err(crate::device::DeviceError::Protocol(crate::device::ProtocolError::Nack(crate::device::ResponseByte::ProgramFileError))) => {
				error!("File does not exist: {}", self.file);
				panic!();
			}
			x => x.unwrap(),
		};
	}
}
