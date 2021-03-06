use crate::commands::Runnable;
use anyhow::Context;
use v5_device::device::{filesystem as fs, send};

/// Print a file's metadata.
#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: fs::QualFileName,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;
		let send_data = send::FileMetadataByName::new(&self.file);
		let metadata = dev.get_file_metadata_by_name(&send_data).context("Getting file metadata")?.context("File does not exist")?;
		println!("Size: {}", metadata.size);
		println!("Address: 0x{:0>8x}", metadata.address);
		println!("File type: {}", metadata.file_type);
		println!("Last modified: {}", metadata.timestamp);
		println!("Version: {}", metadata.version);
		println!("Is link: {}", metadata.is_link());
		if let Some((link_category, link_name)) = metadata.get_link() {
			println!("Linked category: {}", link_category);
			println!("Linked filename: {}", link_name);
		}
		Ok(())
	}
}
