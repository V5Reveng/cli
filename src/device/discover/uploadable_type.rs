#[derive(Debug, Clone, Copy)]
pub enum UploadableType {
	Brain,
	Controller,
}
impl std::fmt::Display for UploadableType {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		fmt.write_str(match self {
			UploadableType::Brain => "brain",
			UploadableType::Controller => "controller",
		})
	}
}
