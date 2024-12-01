
pub fn is_whitespace(c: u8) -> bool {
	return c == b'\t' || c == b' ' || c == b'\r';
}

pub fn is_special(c: u8) -> bool {
	return c == b':' || c == b',' || c == b';' || c == b'.' || c == b'=' || c == b'\n' || is_whitespace(c);
}

pub fn is_numeric(c: u8) -> bool {
	return c >= b'0' && c <= b'9';
}

pub fn to_string(bytes: &[u8]) -> String {
	return String::from_utf8(bytes.to_vec()).unwrap();
}

pub fn get_text_pos(bytes: &[u8], pos: usize) -> (usize, usize) {
	let mut x = 1;
	let mut y = 1;
	for i in 0..pos {
		match bytes[i] {
			b'\n' => {
				y += 1;
				x = 1;
			}
			b'\t' => {
				x = (x / 4 + 1) * 4;
			}
			_ => {
				x += 1;
			}
		}
	}
	return (y, x);
}

const HEX: &[u8] = b"0123456789abcdef";

pub fn to_hex(bytes: &[u8]) -> String {
	let mut out = String::new();
	for c in bytes.iter().copied() {
		out.push(HEX[(c >> 4) as usize] as char);
		out.push(HEX[(c & 15) as usize] as char);
	}
	return out;
}

