use crate::commands::Runnable;
use crate::device::{filesystem as fs, send};

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: Option<String>,
	/// The category of the file. Can be user, system, pros, rms, mw
	#[clap(long, short, default_value_t = Default::default())]
	category: fs::Category,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		if let Some(file) = self.file {
			// print info for the file
			let (file_name, _) = crate::commands::string_to_file_name_and_type(&file);
			let send_data = send::FileMetadataByName::new(self.category, file_name);
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
		} else {
			// list files
			let files = dev.list_all_files(self.category).unwrap();
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
			0
		}
	}
}
