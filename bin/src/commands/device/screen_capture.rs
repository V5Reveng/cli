use crate::commands::Runnable;
use std::fs::File;
use std::io::stdout;
use std::io::{self, Write};
use std::path::PathBuf;
use v5_device::device::Device;

/// Take a screen capture of the device.
#[derive(clap::Parser)]
pub struct Args {
	/// Where to write the screen capture, in PNG format.
	/// If not specified, or "-" is specified, output the data on standard output.
	output: Option<PathBuf>,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence<Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);

		let stream: Box<dyn Write> = match self.output {
			Some(ref path) if path.as_os_str() != "-" => Box::new(File::create(path).expect("Creating output file")),
			_ => Box::new(stdout()),
		};
		let mut stream = ScreenCapturePipeline::new(stream);
		dev.capture_screen(&mut stream).expect("Capturing screen");
		0
	}
}

/// The pipeline:
///
/// 0. Input a `Device::SCREEN_WIDTH` x `Device::SCREEN_HEIGHT` raw image blob in BGRA (due to endianness) format.
/// 1. Crop its width to `Device::ACTUAL_SCREEN_WIDTH`.
/// 2. Rearrange the data to RGB, dropping the alpha channel.
/// 3. Output it as PNG data to the `Write`r provided at initialization.
struct ScreenCapturePipeline {
	output: png::StreamWriter<'static, Box<dyn Write>>,
	current_x: usize,
}

impl ScreenCapturePipeline {
	pub fn new(output: Box<dyn std::io::Write>) -> Self {
		let mut encoder = png::Encoder::new(output, Device::ACTUAL_SCREEN_WIDTH as u32, Device::SCREEN_HEIGHT as u32);
		encoder.set_color(png::ColorType::Rgb);
		encoder.set_depth(png::BitDepth::Eight);
		let output = encoder.write_header().expect("Writing PNG header").into_stream_writer().expect("Converting PNG writer to stream writer");
		Self { output, current_x: 0 }
	}
}

impl Write for ScreenCapturePipeline {
	fn write(&mut self, data: &[u8]) -> io::Result<usize> {
		// The image is made up of u32 units, so we will never get a chunk less than 4 bytes as long as we always accept 4 bytes. Assert this.
		assert!(data.len() >= 4);
		if self.current_x < Device::ACTUAL_SCREEN_WIDTH {
			// BGRA to RGB
			self.output.write_all(&[data[2], data[1], data[0]])?;
		}
		self.current_x = (self.current_x + 1) % Device::SCREEN_WIDTH;
		Ok(4)
	}
	fn flush(&mut self) -> io::Result<()> {
		self.output.flush()
	}
}
