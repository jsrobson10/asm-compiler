use crate::{compiler::Compiler, instruction::{self, InstType, MathType}};


pub struct Token<'a> {
	pub itype: InstType,
	pub mtype: MathType,
	pub args: Vec<&'a [u8]>,
}

impl Token<'_> {
	pub fn new<'a>(itype: InstType, mtype: MathType) -> Token<'a> {
		Token {itype, mtype, args: Vec::new()}
	}

	pub fn process(&self, compiler: &mut Compiler) {
		let mut data: Vec<u8> = Vec::new();

		for arg in self.args.iter().copied() {
			data.push(compiler.to_byte(arg).unwrap());
		}

		compiler.add(self.itype, self.mtype, &data);
	}

	pub fn size(&self) -> u8 {
		return instruction::get_arg_count(self.itype) + 1;
	}
}

