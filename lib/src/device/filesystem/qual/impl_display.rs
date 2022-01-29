use super::{QualFile, QualFileName};
use std::fmt::{self, Display, Formatter};

impl Display for QualFileName {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		write!(formatter, "{}:{}", self.category, self.name)
	}
}

impl Display for QualFile {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		write!(formatter, "{} (type: {})", self.common, self.ty)
	}
}
