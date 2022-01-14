use super::super::{Category, FileName, FileType};
use super::{QualFile, QualFileFromStrError as Error, QualFileName};
use std::str::FromStr;

impl FromStr for QualFileName {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some((category, name)) = s.split_once(':') {
			Ok(Self {
				category: Category::from_str(category).map_err(Self::Err::Category)?,
				name: FileName::from_str(name).map_err(Self::Err::FileName)?,
			})
		} else {
			Ok(Self {
				category: Category::default(),
				name: FileName::from_str(s).map_err(Self::Err::FileName)?,
			})
		}
	}
}

impl FromStr for QualFile {
	type Err = Error;
	// by not implementing this in terms of QualFileName::from_str, we avoid the buffering and unbuffering through FileName instances, instead using &str until we actually need to write the data.
	// &str is not actually much smaller than a FileName (16 bytes vs 24), but it allows for the easy and lightweight creation and manipulation of substrings.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut ret_category = Category::default();
		let mut ret_name = s;
		let ret_ty;

		// if there is a category, the ret_name will be the part after the colon
		// e.g., user:slot_1.ini
		if let Some((category, other)) = s.split_once(':') {
			ret_category = Category::from_str(category).map_err(Self::Err::Category)?;
			ret_name = other;
		}
		// otherwise, the ret_name will be the whole string
		// e.g., slot_1.ini

		// then, using the name from the previous section, attempt to get the type
		ret_ty = if let Some((_stem, ty)) = ret_name.rsplit_once('.') { ty } else { "bin" };

		Ok(Self {
			common: QualFileName {
				category: ret_category,
				name: FileName::from_str(ret_name).map_err(Self::Err::FileName)?,
			},
			ty: FileType::from_str(ret_ty).map_err(Self::Err::FileType)?,
		})
	}
}
