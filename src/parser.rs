use std::fs;

use crate::{compiler::Compiler, instruction::{get_named_inst_type, get_named_math_type, MathType}, text, token::Token};

struct State {
	at: usize,
}

struct Parser {
	src: Vec<u8>,
}

pub fn from_source(src: Vec<u8>, compiler: &mut Compiler) -> Result<(), String> {
	let parser = Parser {src};
	return parser.parse(compiler);
}

pub fn from_file(path: &str, compiler: &mut Compiler) -> Result<(), String> {
	let parser = Parser {src: match fs::read(path) {
		Ok(v) => v,
		Err(e) => return Err(e.to_string()),
	}};
	return parser.parse(compiler);
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

	fn parse_instruction(&self, state: &mut State, iname: &[u8], mname: Option<&[u8]>) -> Result<Token, String> {
		let mut token = Token::new(
			match get_named_inst_type(&iname) {
				Some(v) => v,
				None => return Err(format!("Instruction {} does not exist", text::to_string(iname))),
			},
			match mname {
				Some(v) => match get_named_math_type(&v) {
					Some(v) => v,
					None => return Err(format!("Math type {} does not exist", text::to_string(v))),
				},
				None => MathType::Zero,
			}
		);

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

		return Ok(token);
	}

	fn parse_symbol(&self, state: &mut State, compiler: &mut Compiler, name: &[u8]) -> Result<u8, String> {
		self.skip_whitespaces(state);
		let word = self.get_word(state);
		if word.len() > 0 {
			let v = match compiler.to_byte(word) {
				Some(v) => v,
				None => return Err(format!("Value or symbol {} is not valid", text::to_string(name))),
			};
			compiler.new_symbol(&name, v);
			return Ok(v);
		}
		else {
			let c = self.get_next(state);
			match c {
				Some(b'.') => {
					state.at += 1;
					return Ok(compiler.new_variable(&name));
				}
				Some(v) => return Err(format!("Expected '.', Received '{}'", v as char)),
				None => return Err(format!("Expected '.', Received none")),
			};
		}
	}

	fn format_error(&self, state: &State, err: String) -> String {
		let (x, y) = text::get_text_pos(&self.src, state.at);
		return format!("Compilation error at {}:{}. {}", x, y, err);
	}

	fn parse(&self, compiler: &mut Compiler) -> Result<(), String> {
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
					return Err(self.format_error(&state, "Unexpected token ';'".to_string()));
				}
				Some(b'=') => {
					state.at += 1;
					match self.parse_symbol(&mut state, compiler, name) {
						Ok(_) => (),
						Err(v) => return Err(self.format_error(&state, v)),
					}
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
					let token = match self.parse_instruction(&mut state, name, mname) {
						Ok(v) => v,
						Err(v) => return Err(self.format_error(&state, v)),
					};
					token_at += token.size();
					tokens.push(token);
					continue;
				}
				None => return Err(self.format_error(&state, "End of file".to_string())),
			}
		}

		for token in tokens {
			if let Err(err) = token.process(compiler) {
				return Err(self.format_error(&state, err));
			}
		}

		return Ok(());
	}
}

