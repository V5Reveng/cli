use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum CategoryFromStrError {
	UnknownCategory,
}
impl Display for CategoryFromStrError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		let s = match self {
			Self::UnknownCategory => "unknown category",
		};
		formatter.write_str(s)
	}
}
impl std::error::Error for CategoryFromStrError {}
