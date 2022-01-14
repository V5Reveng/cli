use super::{Address, FileSize, QualFileName, Target, TimeStamp, TransferCompleteAction};

#[derive(Default)]
pub struct ReadArgs {
	pub target: Target,
	pub address: Option<Address>,
	pub size: Option<FileSize>,
}

#[derive(Default)]
pub struct WriteArgs {
	pub action: TransferCompleteAction,
	pub target: Target,
	pub address: Option<Address>,
	pub overwrite: bool,
	pub timestamp: TimeStamp,
	pub linked_file: Option<QualFileName>,
}

#[derive(Default)]
pub struct DeleteArgs {
	pub include_linked: bool,
}
