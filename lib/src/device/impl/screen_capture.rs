use crate::device::{filesystem as fs, Device, Result};

impl Device {
	pub const ACTUAL_SCREEN_WIDTH: usize = 480;
	pub const SCREEN_WIDTH: usize = 512;
	pub const SCREEN_HEIGHT: usize = 272;
	pub const SCREEN_CHANNELS: usize = 4; // ARGB
	pub const SCREEN_TOTAL_SIZE: usize = Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT * Self::SCREEN_CHANNELS;

	pub fn prepare_screen_capture(&mut self) -> Result<()> {
		self.ext_command_no_data::<()>(0x28)
	}
	pub fn receive_screen_capture(&mut self, output_stream: &mut dyn std::io::Write) -> Result<()> {
		self.set_transfer_channel(fs::Channel::FileTransfer)?;
		self.read_file_to_stream(
			output_stream,
			&fs::QualFile {
				common: fs::QualFileName {
					category: fs::Category::SYSTEM,
					name: fs::FileName::default(),
				},
				ty: fs::FileType::default(),
			},
			&fs::ReadArgs {
				target: fs::Target::Screen,
				address: Some(0),
				size: Some(Self::SCREEN_TOTAL_SIZE as u32),
				ignore_crc: true,
			},
		)?;
		self.set_transfer_channel(fs::Channel::Pit)?;
		Ok(())
	}
}
