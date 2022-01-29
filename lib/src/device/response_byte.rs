use encde::{Decode, Encode};
use std::fmt::{self, Display, Formatter};

#[repr(u8)]
#[must_use = "This may be a NACK, which should be handled"]
#[derive(Encode, Decode, PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum ResponseByte {
	/// No error occurred.
	Ack = 0x76,
	/// An unspecified error occurred.
	GeneralNack = 0xff,
	/// Our CRC was invalid.
	ReceivedCrcError = 0xce,
	/// The payload is too small.
	// FIXME: needs more info
	PayloadTooSmall = 0xd0,
	/// The requested data would be too large to transfer.
	RequestedTransferTooLarge = 0xd1,
	/// The program CRC was invalid.
	// FIXME: this is a guess
	ProgramCrcError = 0xd2,
	/// An error occurred relating to program files.
	// FIXME: needs more info
	ProgramFileError = 0xd3,
	/// There was an attempt to upload or download uninitialized data.
	// FIXME: this is a guess
	UninitializedUploadDownload = 0xd4,
	/// The initialization was invalid for the file transfer type.
	// FIXME: this is a guess
	InitInvalidForFunction = 0xd5,
	/// The data is not aligned to 4 bytes.
	DataNotAligned = 0xd6,
	/// The file transfer packet address does not match the expected.
	PacketAddressWrong = 0xd7,
	/// Upon completion of the file transfer, the amount of received data did not match the length specified at the start of the file transfer.
	DownloadedLengthWrong = 0xd8,
	/// The requested file does not exist.
	Enoent = 0xd9,
	/// There is no space left on the device.
	Enospc = 0xda,
	/// The file already exists and the overwrite option was not specified.
	Eexist = 0xdb,
}

impl Display for ResponseByte {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		use ResponseByte::*;
		f.write_str(match self {
			Ack => "Ack",
			GeneralNack => "General Nack",
			ReceivedCrcError => "Received CRC error",
			PayloadTooSmall => "Payload too small",
			RequestedTransferTooLarge => "Requested transfer too large",
			ProgramCrcError => "Program CRC error",
			ProgramFileError => "Program file error",
			UninitializedUploadDownload => "Uninitialized upload download",
			InitInvalidForFunction => "Initialization invalid for function",
			DataNotAligned => "Data not aligned",
			PacketAddressWrong => "Packet address wrong",
			DownloadedLengthWrong => "Downloaded length wrong",
			Enoent => "No such file or directory",
			Enospc => "No space left on device",
			Eexist => "File exists",
		})
	}
}
