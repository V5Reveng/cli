use crate::commands::Runnable;
use crate::util::diff::files_differ_and_f1_len;
use crate::util::temp_dir::TempDir;
use log::{error, warn};
use std::io::{Read, Seek};
use std::{fs, process};
use v5_device::crc::CrcComputable;
use v5_device::device::{filesystem as dev_fs, Device};

/// Edit a file using $EDITOR.
#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: dev_fs::QualFile,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence<Device>) -> u32 {
		let mut dev = crate::commands::unwrap_device_presence(dev);

		// do this now so it fails before we do IO
		let editor = std::env::var("EDITOR").expect("Reading EDITOR env var");

		// prepare temporary environment
		let work_dir = TempDir::new().expect("Creating temporary directory");
		let file_name_str = self.file.common.name.as_str().unwrap();
		let to_edit_name = work_dir.join(file_name_str);
		let backup_name = work_dir.join(file_name_str.to_owned() + ".old");

		// read into the temporary file
		{
			let mut to_edit_file = fs::File::create(&to_edit_name).expect("Creating local copy file");
			dev.read_file_to_stream(&mut to_edit_file, &self.file, &dev_fs::ReadArgs::default()).expect("Could not read file from device");
		}

		// make a backup
		{
			fs::copy(&to_edit_name, &backup_name).expect("Creating backup file");
			let mut new_perms = fs::metadata(&backup_name).expect("Getting metadata for backup file").permissions();
			new_perms.set_readonly(true);
			fs::set_permissions(&backup_name, new_perms).expect("Setting backup file read-only");
		}

		// run the editor
		{
			let mut proc = process::Command::new(editor).arg(&to_edit_name).spawn().expect("Running editor");
			proc.wait().expect("Waiting for editor to close");
		}
		let edited_name = to_edit_name;

		// check for a difference, exiting if none or if the edited file no longer exists
		if !edited_name.is_file() {
			error!("Edited file no longer exists or is not a file.");
			return 1;
		}
		let mut edited_file = fs::File::open(&edited_name).expect("Opening edited file");
		let (files_are_different, edited_len) = {
			let mut backup_file = fs::File::open(&backup_name).expect("Opening backup file");
			files_differ_and_f1_len(&mut edited_file, &mut backup_file).expect("Checking file difference")
		};
		if !files_are_different {
			warn!("File was unchanged.");
			return 1;
		}

		// write back the edited file
		{
			let edited_crc = crc32_from_file(&mut edited_file).expect("Calculating edited file CRC");
			edited_file.rewind().expect("Rewinding edited file to beginning");
			dev.write_file_from_stream(
				&mut edited_file,
				&self.file,
				edited_len.try_into().expect("Edited file is too large"),
				edited_crc,
				&dev_fs::WriteArgs { overwrite: true, ..Default::default() },
			)
			.expect("Writing edited file");
		}

		0
	}
}

fn crc32_from_file(f: &mut fs::File) -> std::io::Result<u32> {
	f.rewind()?;
	let mut buf = [0u8; 1024];
	let mut crc = 0u32;
	loop {
		let amount_read = f.read(&mut buf)?;
		crc.update_crc(&buf[0..amount_read]);
		if amount_read < buf.len() {
			break;
		}
	}
	Ok(crc)
}
