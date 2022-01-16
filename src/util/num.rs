pub fn lenient_u64_from_str(s: &str) -> Result<u64, std::num::ParseIntError> {
	if let Some(s) = s.strip_prefix("0x") {
		u64::from_str_radix(s, 16)
	} else if let Some(s) = s.strip_prefix("0o") {
		u64::from_str_radix(s, 8)
	} else if let Some(s) = s.strip_prefix("0b") {
		u64::from_str_radix(s, 2)
	} else {
		s.parse()
	}
}
