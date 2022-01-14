#[derive(Debug)]
pub enum CategoryFromStrError {
	UnknownCategory,
}
impl std::fmt::Display for CategoryFromStrError {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		let s = match self {
			Self::UnknownCategory => "unknown category",
		};
		formatter.write_str(s)
	}
}
impl std::error::Error for CategoryFromStrError {}
