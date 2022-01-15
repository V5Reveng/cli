use super::Category;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum CategoryFromStrError {
	UnknownCategory,
	TooLarge,
}

impl Display for CategoryFromStrError {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		match self {
			Self::UnknownCategory => {
				write!(formatter, "unknown category. possible categories are `default` ({}), ", Category::default(),)?;
				for &category in Category::named() {
					write!(formatter, "`{}` ({1}/0x{1:02x}), ", category, category.into_inner())?;
				}
				writeln!(formatter, "or a number.")
			}
			Self::TooLarge => {
				write!(formatter, "numbered category was too large. the range is {} to {} (hex {0:02x} to {1:02x}).", Category::MIN, Category::MAX)
			}
		}
	}
}

impl std::error::Error for CategoryFromStrError {}
