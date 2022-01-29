//! File identifiers "qualified" with categories.

use super::{Category, FileName, FileType};

pub mod error;
mod impl_display;
mod impl_from_str;

pub use error::QualFileFromStrError;

/// A qualified file name, that is, one with a category.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct QualFileName {
	pub category: Category,
	pub name: FileName,
}

/// A qualified file, that is, one with a category and type.
#[derive(Debug, Hash)]
pub struct QualFile {
	pub common: QualFileName,
	pub ty: FileType,
}
