use super::FixedString;

impl<const N: usize> std::fmt::Debug for FixedString<N> {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(formatter, "FixedString<{}> ", N)?;
		self.as_str().map_err(|_| std::fmt::Error::default())?.fmt(formatter)
	}
}
impl<const N: usize> std::fmt::Display for FixedString<N> {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.as_str().map_err(|_| std::fmt::Error::default())?.fmt(formatter)
	}
}
