use crate::commands::Runnable;
use crate::device::filesystem as fs;
use log::error;
use std::io::stdout;

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: String,
	/// The category of the file. Can be user, system, pros, rms, mw
	#[clap(long, short, default_value_t = Default::default())]
	category: fs::Category,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let (file_name, file_type) = crate::commands::string_to_file_name_and_type(&self.file);
		let contents = dev.read_file_to_stream(
			&mut stdout(),
			&crate::device::filesystem::ReadArgs {
				file_name,
				file_type,
				category: self.category,
				..Default::default()
			},
		);
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
