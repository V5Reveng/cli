#[derive(Debug, Clone, Copy)]
pub enum UploadableType {
	Brain,
	Controller,
}
impl std::fmt::Display for UploadableType {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		use UploadableType::*;
		fmt.write_str(match self {
			Brain => "brain",
			Controller => "controller",
		})
	}
}
