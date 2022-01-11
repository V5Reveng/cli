use crate::commands::Runnable;
use crate::device::{filesystem as fs, send};

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
			let (file_name, _) = crate::commands::string_to_file_name_and_type(&file);
			let send_data = send::FileMetadataByName::new(fs::Category::default(), file_name);
			let metadata = dev.get_file_metadata_by_name(&send_data).unwrap();
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
		} else {
			// list files
			let files = dev.list_all_files(fs::Category::default()).unwrap();
			println!("Num files: {}", files.len());
			println!("Address      Mtime                       Version    Size  Type  Name\n");
			for metadata in files {
				println!(
					"0x{address:0>8x}  {mtime}  {version: >8}  {size: >4}  {ty: <4}  {name}",
					size = metadata.size,
					address = metadata.address,
					ty = metadata.file_type,
					mtime = metadata.timestamp,
					version = metadata.version,
					name = metadata.name
				);
			}
		}
	}
}
