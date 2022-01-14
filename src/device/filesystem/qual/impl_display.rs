use super::{QualFile, QualFileName};

impl std::fmt::Display for QualFileName {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "{}:{}", self.category, self.name)
	}
}

impl std::fmt::Display for QualFile {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "{} (type: {})", self.common, self.ty)
	}
}
