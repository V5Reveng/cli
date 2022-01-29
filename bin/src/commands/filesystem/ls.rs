use crate::commands::Runnable;
use v5_device::device::{filesystem as fs, receive, Device};

/// List files in a category, or all files.
#[derive(clap::Parser)]
pub struct Args {
	/// If category is not specified, only files in named categories are listed.
	category: Option<fs::Category>,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence<Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);
		match self.category {
			Some(category) => list_in_category(&mut dev, category),
			None => list_all_categories(&mut dev),
		};
		0
	}
}

/// Prints aligned in a table including a nice header.
fn print_file_list(files: &[receive::FileMetadataByIndex]) {
	println!("Num files: {}", files.len());
	println!("Address     Mtime                       Version  Size   Type  Name\n");
	for metadata in files {
		println!(
			"0x{address:0>8x}  {mtime}  {version: >8}  {size: <5}  {ty: <4}  {name}",
			size = metadata.size,
			address = metadata.address,
			ty = metadata.file_type,
			mtime = metadata.timestamp,
			version = metadata.version,
			name = metadata.name
		);
	}
}

fn list_in_category(dev: &mut Device, category: fs::Category) {
	let files = dev.list_all_files(category).unwrap();
	print_file_list(&files);
}

/// List the files in all *named* categories.
fn list_all_categories(dev: &mut Device) {
	for &category in fs::Category::named() {
		let files = dev.list_all_files(category).unwrap();
		println!("Category: {}", category);
		print_file_list(&files);
		println!();
	}
}
