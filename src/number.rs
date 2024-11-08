
pub fn parse_radix(bytes: &[u8], radix: u8) -> Option<u8> {
	let mut num = 0;
	for c in bytes.iter().copied() {
		let v;
		if c >= b'0' && c <= b'9' {
			v = c - b'0';
		}
		else if c >= b'a' && c <= b'z' {
			v = c - b'a' + 10;
		}
		else if c >= b'A' && c <= b'z' {
			v = c - b'A' + 10;
		}
		else {
			return None;
		}
		if v >= radix {
			return None;
		}
		num = num * radix + v;
	}
	return Some(num);
}

