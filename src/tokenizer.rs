use crate::CELLS;

#[derive(PartialEq)]
pub enum TokenType {
	LoopIn,
	LoopOut,
	IncrementDecrement,
	NextPrevious,
	Input,
	Output
}

pub struct Token {
	pub token_type: TokenType,
	pub count: i64
}

pub fn tokenize(stream: &str) -> Vec<Token> {
	let chars = stream.as_bytes();
	let mut tokens: Vec<Token> = Vec::new();

	for c in chars {
		match c {
			b'+' => {
				if let Some(token) = tokens.last_mut() {
					if token.token_type == TokenType::IncrementDecrement {
						token.count += 1;
						continue;
					}
				}

				tokens.push(Token { token_type: TokenType::IncrementDecrement, count: 1 })
			}
			b'-' => {
				if let Some(token) = tokens.last_mut() {
					if token.token_type == TokenType::IncrementDecrement {
						token.count -= 1;
						continue;
					}
				}

				tokens.push(Token { token_type: TokenType::IncrementDecrement, count: -1 })
			}
			b'>' => {
				if let Some(token) = tokens.last_mut() {
					if token.token_type == TokenType::NextPrevious {
						token.count += 1;
						continue;
					}
				}

				tokens.push(Token { token_type: TokenType::NextPrevious, count: 1 })
			}
			b'<' => {
				if let Some(token) = tokens.last_mut() {
					if token.token_type == TokenType::NextPrevious {
						token.count -= 1;
						continue;
					}
				}

				tokens.push(Token { token_type: TokenType::NextPrevious, count: -1 })
			}
			b'[' => tokens.push(Token { token_type: TokenType::LoopIn, count: 0 }),
			b']' => tokens.push(Token { token_type: TokenType::LoopOut, count: 0 }),
			b',' => tokens.push(Token { token_type: TokenType::Input, count: 0 }),
			b'.' => {
				if let Some(token) = tokens.last_mut() {
					if token.token_type == TokenType::Output {
						token.count += 1;
						continue;
					}
				}

				tokens.push(Token { token_type: TokenType::Output, count: 1 })
			}
			_ => ()
		}
	}

	return tokens;
}

pub fn optimize(tokens: Vec<Token>) -> Vec<Token> {
	let mut optimized: Vec<Token> = Vec::new();

	for mut token in tokens {
		if token.token_type == TokenType::IncrementDecrement {
			if token.count == 0 {
				continue;
			} else if token.count < 0 {
				token.count = -(-token.count % 256);
			} else {
				token.count %= 256;
			}
		}

		if token.token_type == TokenType::NextPrevious {
			if token.count == 0 {
				continue;
			} else if token.count < 0 {
				token.count = -(-token.count % CELLS as i64);
			} else {
				token.count %= CELLS as i64;
			}
		}

		optimized.push(token);
	}

	return optimized;
}