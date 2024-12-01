use std::{env, fs};

pub mod text;
pub mod compiler;
pub mod instruction;
pub mod parser;
pub mod number;
pub mod token;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut compiler = compiler::Compiler::new();

	if let Err(err) = parser::from_file(&args[1], &mut compiler) {
		println!("{}", err);
		return;
	}

	if args.len() == 3 {
		fs::write(&args[2], compiler.get_data()).unwrap();
		println!("Written binary to {}", &args[2]);
	}

	else {
		println!("{}", text::to_hex(compiler.get_data()));
	}
}

