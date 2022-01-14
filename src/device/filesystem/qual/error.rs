use super::super::{CategoryFromStrError, FixedStringFromStrError};

#[derive(Debug)]
pub enum QualFileFromStrError {
	Category(CategoryFromStrError),
	FileName(FixedStringFromStrError),
	FileType(FixedStringFromStrError),
}
impl std::fmt::Display for QualFileFromStrError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Category(e) => {
				formatter.write_str("invalid category: ")?;
				std::fmt::Display::fmt(e, formatter)
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
