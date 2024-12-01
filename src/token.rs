use crate::{compiler::Compiler, instruction::{InstType, MathType}, text};


pub struct Token<'a> {
	pub itype: InstType,
	pub mtype: MathType,
	pub args: Vec<&'a [u8]>,
}

impl Token<'_> {
	pub fn new<'a>(itype: InstType, mtype: MathType) -> Token<'a> {
		Token {itype, mtype, args: Vec::new()}
	}

	pub fn process(&self, compiler: &mut Compiler) -> Result<(), String> {
		let mut data: Vec<u8> = Vec::new();

		for arg in self.args.iter().copied() {
			data.push(match compiler.to_byte(arg) {
				Some(v) => v,
				None => return Err(format!("Could not process argument {}", text::to_string(arg))),
			});
		}

		return compiler.add(self.itype, self.mtype, &data);
	}

	pub fn size(&self) -> u8 {
		return self.itype.arg_count() + 1;
	}
}

