use core::any::type_name;

pub enum Presence<T> {
	None,
	One(T),
	Many(Vec<T>),
}

impl<T> From<Vec<T>> for Presence<T> {
	fn from(mut items: Vec<T>) -> Self {
		match items.len() {
			0 => Self::None,
			1 => Self::One(items.pop().unwrap()),
			_ => Self::Many(items),
		}
	}
}

impl<T> From<Option<T>> for Presence<T> {
	fn from(opt: Option<T>) -> Self {
		match opt {
			None => Self::None,
			Some(x) => Self::One(x),
		}
	}
}

enum NotOne<T> {
	None,
	Many(Vec<T>),
}

impl<T> From<Presence<T>> for Result<T, NotOne<T>> {
	fn from(pres: Presence<T>) -> Self {
		match pres {
			Presence::<T>::None => Err(NotOne::<T>::None),
			Presence::<T>::One(item) => Ok(item),
			Presence::<T>::Many(items) => Err(NotOne::<T>::Many(items)),
		}
	}
}

impl<T: std::fmt::Debug> Presence<T> {
	pub fn expect_one(self) -> T {
		match self {
			Self::None => panic!("Expected one {} present, found none", type_name::<T>()),
			Self::One(item) => item,
			Self::Many(ref items) => {
				panic!("Expected one {} present, found many:\n{:?}", type_name::<T>(), items);
			}
		}
	}
}
