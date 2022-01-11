use crate::commands::Runnable;
use crate::crc::CrcComputable;
use crate::device::filesystem as dev_fs;
use crate::temp_dir::TempDir;
use log::{error, warn};
use std::io::{self, Read, Seek};
use std::{fs, process};

#[derive(clap::Parser)]
pub struct Args {
	/// Remote filename.
	file: String,
	/// The category of the file. Can be user, system, pros, rms, mw
	#[clap(long, short, default_value_t = Default::default())]
	category: dev_fs::Category,
}

impl Runnable for Args {
	fn run(self, dev: crate::presence::Presence<crate::device::Device>) {
		let mut dev = crate::commands::unwrap_device_presence(dev);

		// do this now so it fails before we do IO
		let (remote_file_name, remote_file_type) = crate::commands::string_to_file_name_and_type(&self.file);
		let editor = std::env::var("EDITOR").expect("Reading EDITOR env var");

		// prepare temporary environment
		let work_dir = TempDir::new().expect("Creating temporary directory");
		let to_edit_name = work_dir.join(&self.file);
		let backup_name = work_dir.join(self.file + ".old");

		// read into the temporary file
		{
			let mut to_edit_file = fs::File::create(&to_edit_name).expect("Creating local copy file");
			dev
				.read_file_to_stream(
					&mut to_edit_file,
					&dev_fs::ReadArgs {
						file_name: remote_file_name,
						file_type: remote_file_type,
						category: self.category,
						..Default::default()
					},
				)
				.expect("Could not read file from device");
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
			return;
		}
		let mut edited_file = fs::File::open(&edited_name).expect("Opening edited file");
		let (files_are_different, edited_len) = {
			let mut backup_file = fs::File::open(&backup_name).expect("Opening backup file");
			files_differ_and_f1_len(&mut edited_file, &mut backup_file).expect("Checking file difference")
		};
		if !files_are_different {
			warn!("File was unchanged.");
			return;
		}

		// write back the edited file
		{
			edited_file.seek(io::SeekFrom::Start(0)).expect("Rewinding edited file to beginning");
			let edited_crc = crc32_from_file(&mut edited_file).expect("Calculating edited file CRC");
			edited_file.seek(io::SeekFrom::Start(0)).expect("Rewinding edited file to beginning");
			dev
				.write_file_from_stream(
					&mut edited_file,
					edited_len.try_into().expect("Edited file is too large"),
					edited_crc,
					&dev_fs::WriteArgs {
						file_name: remote_file_name,
						file_type: remote_file_type,
						category: self.category,
						overwrite: true,
						..Default::default()
					},
				)
				.expect("Writing edited file");
		}
	}
}

fn files_differ_and_f1_len(f1: &mut fs::File, f2: &mut fs::File) -> std::io::Result<(bool, usize)> {
	let f1_len = f1.metadata()?.len();
	let f2_len = f2.metadata()?.len();
	if f1_len != f2_len {
		Ok((true, f1_len.try_into().unwrap()))
	} else {
		// files are the same size
		const BUF_SIZE: usize = 512;
		let mut f1_buf = [0u8; BUF_SIZE];
		let mut f2_buf = [0u8; BUF_SIZE];
		loop {
			let f1_read_len = f1.read(&mut f1_buf)?;
			let f2_read_len = f2.read(&mut f2_buf)?;
			let read_len = match std::cmp::Ord::cmp(&f1_read_len, &f2_read_len) {
				std::cmp::Ordering::Equal => f1_read_len,
				std::cmp::Ordering::Less => {
					f1.read_exact(&mut f1_buf[f1_read_len..f2_read_len])?;
					f2_read_len
				}
				std::cmp::Ordering::Greater => {
					f2.read_exact(&mut f2_buf[f2_read_len..f1_read_len])?;
					f1_read_len
				}
			};
			for i in 0..read_len {
				if f1_buf[i] != f2_buf[i] {
					return Ok((false, f1_len.try_into().unwrap()));
				}
			}
			if read_len < BUF_SIZE {
				return Ok((true, f1_len.try_into().unwrap()));
			}
		}
	}
}

fn crc32_from_file(f: &mut fs::File) -> std::io::Result<u32> {
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
