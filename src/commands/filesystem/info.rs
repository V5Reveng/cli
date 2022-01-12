use crate::commands::Runnable;
use crate::device::{filesystem as fs, send};

#[derive(clap::Parser)]
pub struct Args {
	file: fs::QualFileName,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		let send_data = send::FileMetadataByName::new(&self.file);
		let metadata = dev.get_file_metadata_by_name(&send_data).unwrap().expect("File does not exist");
		println!("Size: {}", metadata.size);
		println!("Address: 0x{:0>8x}", metadata.address);
		println!("File type: {}", metadata.file_type);
		println!("Last modified: {}", metadata.timestamp);
		println!("Version: {}", metadata.version);
		let is_link = metadata.linked_category != fs::Category::None;
		println!("Is link? {}", is_link);
		if is_link {
			println!("Linked category: {}", metadata.linked_category);
			println!("Linked filename: {}", metadata.linked_name);
		}
		0
	}
}
