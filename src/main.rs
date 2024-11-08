use std::env;


pub mod text;
pub mod compiler;
pub mod instruction;
pub mod parser;
pub mod number;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut compiler = compiler::Compiler::new();
	parser::from_file(&args[1], &mut compiler);
	println!("Generated: {}", text::to_hex(compiler.get_data()));
}

