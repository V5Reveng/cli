use super::super::{CategoryFromStrError, FixedStringFromStrError};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum QualFileFromStrError {
	Category(CategoryFromStrError),
	FileName(FixedStringFromStrError),
	FileType(FixedStringFromStrError),
}

impl Display for QualFileFromStrError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self {
			Self::Category(e) => {
				formatter.write_str("invalid category: ")?;
				e.fmt(formatter)
			}
			Self::FileName(e) => {
				formatter.write_str("invalid file name: ")?;
				e.fmt(formatter)
			}
			Self::FileType(e) => {
				formatter.write_str("invalid file type: ")?;
				e.fmt(formatter)
			}
		}
	}
}

impl std::error::Error for QualFileFromStrError {}
