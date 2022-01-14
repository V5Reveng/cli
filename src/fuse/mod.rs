#![cfg(target_os = "linux")]

use crate::device::filesystem::{self as fs, Category, FileName, QualFileName};
use crate::device::{receive, send, Device};
use std::collections::HashMap;

type Inode = u64;

const NUM_CATEGORIES: usize = u8::MAX as usize;
struct FileData {
	data: Vec<u8>,
	inode: Inode,
}
type FileTree = HashMap<QualFileName, FileData>;

enum InodeClass {
	Root,
	/// The contents of a category
	Category(Category),
	/// A file itself
	File,
	Invalid,
}
impl InodeClass {
	pub fn classify(inode: Inode) -> Self {
		// All inodes less than 0x200 (512) are reserved.
		// within 0x00 to 0xff inclusive, 0x01 = root and the rest are unused
		// within 0x100 to 0x1ff inclusive, each represents a category directory
		match inode {
			fuse::FUSE_ROOT_ID => Self::Root,
			0x100..=0x1ff => Self::Category(Category((inode - 0x100).try_into().unwrap())),
			x if x >= 0x200 => Self::File,
			_ => Self::Invalid,
		}
	}
}

type NumLookups = u64;
struct InodeDescriptor {
	pub num_lookups: NumLookups,
	pub name: QualFileName,
}

#[derive(Default)]
struct InodeStorage(HashMap<Inode, InodeDescriptor>);
impl InodeStorage {
	pub fn increment_references(&mut self, inode: Inode, amount: NumLookups) {
		let mut descriptor = self.0.get_mut(&inode).unwrap();
		descriptor.num_lookups += amount;
	}
	/// returns the evicted file name if it was evicted
	pub fn decrement_references(&mut self, inode: Inode, amount: NumLookups) -> Option<QualFileName> {
		let mut descriptor = self.0.get_mut(&inode).unwrap();
		if descriptor.num_lookups < amount {
			panic!("Tried to decrement inode reference count by more references than exist");
		}
		descriptor.num_lookups -= amount;
		if descriptor.num_lookups == 0 {
			Some(self.0.remove(&inode).unwrap().name)
		} else {
			None
		}
	}
}

pub struct Server {
	device: Device,
	cache: FileTree,
	/// We have to spoof the existence of inodes by storing them here.
	inode_storage: InodeStorage,
}

impl Server {
	pub fn from_device(device: Device) -> Self {
		Self {
			device,
			cache: Default::default(),
			inode_storage: Default::default(),
		}
	}
}

use fuse::*;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};
type CInt = libc::c_int;
use libc::ENOENT;
const TTL: Duration = Duration::from_secs(1);

impl Server {
	fn category_inode(category: Category) -> Inode {
		0x100 + (category.into_inner() as Inode)
	}
	fn category_attr(category: Category) -> FileAttr {
		FileAttr {
			ino: Self::category_inode(category),
			size: 0,
			blocks: 0,
			atime: UNIX_EPOCH,
			mtime: UNIX_EPOCH,
			ctime: UNIX_EPOCH,
			crtime: UNIX_EPOCH,
			kind: FileType::Directory,
			perm: 0o777,
			nlink: 2,
			uid: 0,
			gid: 0,
			rdev: 0,
			flags: 0,
		}
	}
	fn fixed_string_err_to_code(err: fs::FixedStringFromStrError) -> CInt {
		use fs::FixedStringFromStrError as E;
		match err {
			E::TooLong => libc::ENAMETOOLONG,
			// By passing through null-terminated land, this is guaranteed not to occur. Handle it anyway 🤠
			E::ContainsNul { .. } => libc::ENOENT,
		}
	}

	fn lookup_root(&mut self, name: &OsStr) -> Result<FileAttr, CInt> {
		match name.to_str().and_then(|name| Category::from_str(name).ok()) {
			Some(category) => Ok(Self::category_attr(category)),
			None => Err(ENOENT),
		}
	}
	fn lookup_category(&mut self, category: Category, name: &OsStr) -> Result<FileAttr, CInt> {
		let name = name.as_bytes();
		let name = fs::FileName::try_from(name).map_err(Self::fixed_string_err_to_code)?;
		let send = send::FileMetadataByName::new(&QualFileName { category, name });
		if let Some(metadata) = self.device.get_file_metadata_by_name(&send).unwrap() {
			todo!()
		} else {
			Err(ENOENT)
		}
	}

	fn lookup_result(&mut self, parent: Inode, name: &OsStr) -> Result<FileAttr, CInt> {
		match InodeClass::classify(parent) {
			InodeClass::Root => self.lookup_root(name),
			InodeClass::Category(category) => self.lookup_category(category, name),
			InodeClass::File => todo!(),
			InodeClass::Invalid => Err(ENOENT),
		}
	}
}

impl Filesystem for Server {
	fn init(&mut self, _req: &Request) -> Result<(), CInt> {
		Ok(())
	}
	fn destroy(&mut self, _req: &Request) {}
	fn lookup(&mut self, _req: &Request, parent: Inode, name: &OsStr, reply: ReplyEntry) {
		match self.lookup_result(parent, name) {
			Ok(ref entry) => reply.entry(&TTL, entry, 0),
			Err(code) => reply.error(code),
		}
	}
	fn forget(&mut self, _req: &Request, inode: Inode, num_lookups: NumLookups) {
		if let Some(evicted) = self.inode_storage.decrement_references(inode, num_lookups) {
			let _ = self.cache.remove(&evicted);
		}
	}
}
