pub fn has_bit(num: u64, bit_idx: u8) -> bool {
	num & (1 << bit_idx) == (1 << bit_idx)
}
