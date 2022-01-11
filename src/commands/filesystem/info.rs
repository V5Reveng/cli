use crate::commands::Runnable;
use crate::device::{filesystem as fs, send};
use std::str::FromStr;

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: Option<String>,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		if let Some(file) = self.file {
			// print info for the file
			let send_data = send::FileMetadataByName::new(fs::Category::default(), fs::FileName::from_str(&file).unwrap());
			let metadata = dev.get_file_metadata_by_name(&send_data).unwrap();
			println!("Size: {}", metadata.size);
			println!("Address: {:0>#8x}", metadata.addr);
			println!("File type: {}", metadata.file_type);
			println!("Last modified: {}", metadata.timestamp);
			println!("Version: {:0>#8x}", metadata.version);
			let is_link = metadata.linked_category != fs::Category::None;
			println!("Is link? {}", is_link);
			if is_link {
				println!("Linked category: {}", metadata.linked_category);
				println!("Linked filename: {}", metadata.linked_name);
			}
		} else {
			// list files
			let files = dev.list_all_files(fs::Category::default()).unwrap();
			println!("Num files: {}", files.len());
			println!("Address    Mtime                       Version    Size  Type  Name\n");
			for metadata in files {
				println!(
					"{address:0>#8x}  {mtime}  {version:0>#8x}  {size: >4}  {ty: <4}  {name}",
					size = metadata.size,
					address = metadata.addr,
					ty = metadata.file_type,
					mtime = metadata.timestamp,
					version = metadata.version,
					name = metadata.name
				);
			}
		}
	}
}
