use rand::distributions::{Distribution, Uniform};
use std::fs::{create_dir, remove_dir_all};
use std::path::{Path, PathBuf};

#[repr(transparent)]
pub struct TempDir(PathBuf);
impl Drop for TempDir {
	fn drop(&mut self) {
		remove_dir_all(self).expect("Cleaning up temporary directory");
	}
}

impl std::ops::Deref for TempDir {
	type Target = Path;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl AsRef<Path> for TempDir {
	fn as_ref(&self) -> &Path {
		&self.0
	}
}

impl TempDir {
	pub fn new() -> std::io::Result<Self> {
		const NUM_RAND_CHARS: usize = 24;
		static PREFIX: &str = "tempdir-";
		let mut rng = rand::thread_rng();
		let random_suffix = Uniform::from('a'..'z').sample_iter(&mut rng).take(NUM_RAND_CHARS);
		let mut dirname = String::with_capacity(PREFIX.len() + NUM_RAND_CHARS);
		dirname.push_str(PREFIX);
		dirname.extend(random_suffix);
		let mut path = std::env::temp_dir();
		path.push(dirname);
		create_dir(&path)?;
		Ok(Self(path))
	}
}
