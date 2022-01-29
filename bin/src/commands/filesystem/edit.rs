use crate::commands::Runnable;
use crate::util::diff::files_differ_and_f1_len;
use crate::util::temp_dir::TempDir;
use anyhow::Context;
use log::warn;
use std::io::{Read, Seek};
use std::{fs, process};
use v5_device::crc::CrcComputable;
use v5_device::device::filesystem as dev_fs;

/// Edit a file using $EDITOR.
#[derive(clap::Parser)]
pub struct Args {
	/// Remote file.
	file: dev_fs::QualFile,
}

impl Runnable for Args {
	fn run(self, dev: v5_device::util::presence::Presence) -> anyhow::Result<()> {
		let mut dev = dev.as_result()?;

		// do this now so it fails before we do IO
		let editor = std::env::var("EDITOR").context("Reading EDITOR env var")?;

		// prepare temporary environment
		let work_dir = TempDir::new().context("Creating temporary directory")?;
		let file_name_str = self.file.common.name.as_str()?;
		let to_edit_name = work_dir.join(file_name_str);
		let backup_name = work_dir.join(format!("{}.old", file_name_str));

		// read into the temporary file
		{
			let mut to_edit_file = fs::File::create(&to_edit_name).context("Creating local copy file")?;
			dev.read_file_to_stream(&mut to_edit_file, &self.file, &dev_fs::ReadArgs::default()).context("Could not read file from device")?;
		}

		// make a backup
		{
			fs::copy(&to_edit_name, &backup_name).context("Creating backup file")?;
			let mut new_perms = fs::metadata(&backup_name).context("Getting metadata for backup file")?.permissions();
			new_perms.set_readonly(true);
			fs::set_permissions(&backup_name, new_perms).context("Setting backup file read-only")?;
		}

		// run the editor
		{
			let mut proc = process::Command::new(editor).arg(&to_edit_name).spawn().context("Running editor")?;
			proc.wait().context("Waiting for editor to close")?;
		}
		let edited_name = to_edit_name;

		// check for a difference, exiting if none or if the edited file no longer exists
		if !edited_name.is_file() {
			anyhow::bail!("Edited file no longer exists or is not a file.");
		}
		let mut edited_file = fs::File::open(&edited_name).context("Opening edited file")?;
		let (files_are_different, edited_len) = {
			let mut backup_file = fs::File::open(&backup_name).context("Opening backup file")?;
			files_differ_and_f1_len(&mut edited_file, &mut backup_file).context("Checking file difference")?
		};
		if !files_are_different {
			warn!("File was unchanged.");
			return Ok(());
		}

		// write back the edited file
		{
			let edited_crc = crc32_from_file(&mut edited_file).context("Calculating edited file CRC")?;
			edited_file.rewind().context("Rewinding edited file to beginning")?;
			dev.write_file_from_stream(
				&mut edited_file,
				&self.file,
				edited_len.try_into().context("Edited file is too large")?,
				edited_crc,
				&dev_fs::WriteArgs { overwrite: true, ..Default::default() },
			)
			.context("Writing edited file")?;
		}

		Ok(())
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
