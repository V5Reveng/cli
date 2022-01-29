use std::cmp::{Ord, Ordering};
use std::fs;
use std::io::Read;

/// Returns whether the files differ, along with the size of the first file.
pub fn files_differ_and_f1_len(f1: &mut fs::File, f2: &mut fs::File) -> std::io::Result<(bool, usize)> {
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
			let read_len = match Ord::cmp(&f1_read_len, &f2_read_len) {
				Ordering::Equal => f1_read_len,
				Ordering::Less => {
					f1.read_exact(&mut f1_buf[f1_read_len..f2_read_len])?;
					f2_read_len
				}
				Ordering::Greater => {
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
