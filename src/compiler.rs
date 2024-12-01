
use std::collections::HashMap;

use crate::{instruction::{InstType, MathType}, number, text};

pub struct Compiler {
	symbols: HashMap<Vec<u8>, u8>,
	data: Vec<u8>,
	symbol_at: u8,
}

impl Compiler {
	
	pub fn new() -> Compiler {
		Compiler {
			symbols: HashMap::new(),
			data: Vec::new(),
			symbol_at: 0x80,
		}
	}

	pub fn add(&mut self, itype: InstType, mtype: MathType, data: &[u8]) -> Result<(), String> {
		let expected = itype.arg_count() as usize;
		if expected != data.len() {
			return Err(format!("Bad instruction argument count. Expected {}, got {}.", expected, data.len()));
		}
		self.data.push(((itype as u8) << 4) | (mtype as u8));

		if itype.is_math() {
			if data[0] & 0xf0 != 0xf0 || data[1] & 0xf0 != 0xf0 {
				return Err(format!("Addresses {:#02x} and {:#02x} must only reference math registers (0xf0 -> 0xff).", data[0], data[1]));
			}
			self.data.push(((data[0] & 15) << 4) | (data[1] & 15));
			self.data.push(data[2]);
		}
		else {
			self.data.extend_from_slice(data);
		}

		return Ok(());
	}

	pub fn new_symbol(&mut self, name: &[u8], index: u8) {
		self.symbols.insert(name.to_vec(), index);
	}

	pub fn new_variable(&mut self, name: &[u8]) -> u8 {
		let index = self.symbol_at;
		self.new_symbol(name, index);
		self.symbol_at += 1;
		return index;
	}

	pub fn get_symbol(&self, name: &[u8]) -> Option<u8> {
		match self.symbols.get(name) {
			Some(v) => Some(*v),
			None => None,
		}
	}

	pub fn to_byte(&self, data: &[u8]) -> Option<u8> {
		if data.len() > 1 && data[0] == b'0' {
			let bytes = &data[2..];
			return match data[1] {
				b'x' | b'X' => number::parse_radix(bytes, 16),
				b'o' | b'O' => number::parse_radix(bytes, 8),
				b'b' | b'B' => number::parse_radix(bytes, 2),
				b'd' | b'D' => number::parse_radix(bytes, 10),
				_ => None,
			};
		}
		if data.len() > 0 && text::is_numeric(data[0]) {
			return number::parse_radix(data, 10);
		}
		if data.len() == 0 {
			return None;
		}
		return self.get_symbol(data);
	}

	pub fn get_data(&self) -> &[u8] {
		return &self.data;
	}
}

