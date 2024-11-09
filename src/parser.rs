use std::fs;

use crate::{compiler::Compiler, instruction::{get_named_inst_type, get_named_math_type, MathType}, text, token::Token};

struct State {
	at: usize,
}

struct Parser {
	src: Vec<u8>,
}

pub fn from_source(src: Vec<u8>, compiler: &mut Compiler) {
	let parser = Parser {src};
	parser.parse(compiler);
}

pub fn from_file(path: &str, compiler: &mut Compiler) {
	let parser = Parser {src: fs::read(path).unwrap()};
	parser.parse(compiler);
}

impl Parser {

	fn get_next(&self, state: &mut State) -> Option<u8> {
		if state.at < self.src.len() {
			return Some(self.src[state.at]);
		} else {
			return None;
		}
	}

	fn skip_comment(&self, state: &mut State) {
		while state.at < self.src.len() {
			let c = self.src[state.at];

			if c == b'\n' {
				break;
			}

			state.at += 1;
		}
	}

	fn skip_whitespaces(&self, state: &mut State) {
		while state.at < self.src.len() {
			let c = self.src[state.at];

			if c == b';' {
				state.at += 1;
				self.skip_comment(state);
				continue;
			}

			if !text::is_whitespace(c) {
				break;
			}

			state.at += 1;
		}
	}

	fn skip_whitespaces_and_newlines(&self, state: &mut State) {
		while state.at < self.src.len() {
			let c = self.src[state.at];

			if c == b';' {
				state.at += 1;
				self.skip_comment(state);
				continue;
			}

			if !text::is_whitespace(c) && c != b'\n' {
				break;
			}

			state.at += 1;
		}
	}

	fn get_word(&self, state: &mut State) -> &[u8] {
		let start = state.at;
		while state.at < self.src.len() {
			let c = self.src[state.at];

			if text::is_special(c) {
				break;
			}

			state.at += 1;
		}
		return &self.src[start..state.at];
	}

	fn parse_instruction(&self, state: &mut State, iname: &[u8], mname: Option<&[u8]>) -> Token {
		let mut token = Token::new(
			get_named_inst_type(&iname).unwrap(),
			match mname {
				Some(v) => get_named_math_type(&v).unwrap(),
				None => MathType::Zero,
		});

		while state.at < self.src.len() {
			self.skip_whitespaces(state);
			
			let word = self.get_word(state);
			if word.len() == 0 {
				break;
			}

			token.args.push(word);

			self.skip_whitespaces(state);

			let c = self.get_next(state);
			if !c.is_some_and(|x| x == b',') {
				break;
			}
			
			state.at += 1;
		}

		return token;
	}

	fn parse_symbol(&self, state: &mut State, compiler: &mut Compiler, name: &[u8]) -> u8 {
		self.skip_whitespaces(state);
		let word = self.get_word(state);
		if word.len() > 0 {
			let v = compiler.to_byte(word).unwrap();
			compiler.new_symbol(&name, v);
			return v;
		}
		else {
			let c = self.get_next(state);
			match c {
				Some(b'.') => {
					state.at += 1;
					return compiler.new_variable(&name);
				}
				Some(v) => {
					panic!("Expected '.', Received '{}'", v as char);
				}
				None => {
					panic!("Expected '.', Received none");
				}
			};
		}
	}

	fn parse(&self, compiler: &mut Compiler) {
		let mut state = State {at: 0};
		let mut tokens: Vec<Token> = Vec::new();
		let mut token_at = 0;

		while state.at < self.src.len() {
			self.skip_whitespaces_and_newlines(&mut state);
			let name = self.get_word(&mut state);
			self.skip_whitespaces(&mut state);
			let c = self.get_next(&mut state);
			if name.len() == 0 {
				break;
			}
			match c {
				Some(b':') => {
					state.at += 1;
					compiler.new_symbol(&name, token_at);
					continue;
				}
				Some(b';') => {
					panic!("Unexpected token ';'");
				}
				Some(b'=') => {
					state.at += 1;
					self.parse_symbol(&mut state, compiler, name);
					continue;
				}
				Some(c) => {
					let mname;
					match c {
						b'.' => {
							state.at += 1;
							mname = Some(self.get_word(&mut state));
						}
						_ => {
							mname = None;
						}
					};
					let token = self.parse_instruction(&mut state, name, mname);
					token_at += token.size();
					tokens.push(token);
					continue;
				}
				None => {
					panic!("End of file");
				}
			}
		}

		for token in tokens {
			token.process(compiler);
		}
	}
}

