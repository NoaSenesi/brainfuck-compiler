use crate::CELLS;
use crate::tokenizer::Token;
use crate::tokenizer::TokenType;

pub fn generate_assembly(tokens: Vec<Token>) -> String {
	let mut code = String::new();
	let mut nth_loop = 0;
	let mut nth_ptr_adjust = 0;
	let mut loop_stack: Vec<u32> = Vec::new();

	code.push_str("section .text\n");
	code.push_str("global _start\n\n");
	code.push_str("_start:\n");
	code.push_str("\tmov rbx, 0\n");

	for token in tokens {
		match token.token_type {
			TokenType::IncrementDecrement => {
				if token.count > 0 {
					code.push_str("\tadd byte rbx[cells], ");
					code.push_str(&token.count.to_string());
					code.push_str("\n");
				} else {
					code.push_str("\tsub byte rbx[cells], ");
					code.push_str(&token.count.abs().to_string());
					code.push_str("\n");
				}
			}
			TokenType::NextPrevious => {
				if token.count > 0 {
					code.push_str("\tadd rbx, ");
					code.push_str(&token.count.to_string());
					code.push_str("\n");
					code.push_str("\tcmp rbx, CELLS\n");
					code.push_str("\tjl .ptr_adjust");
					code.push_str(&nth_ptr_adjust.to_string());
					code.push_str("\n");
					code.push_str("\tsub rbx, CELLS\n");
					code.push_str(".ptr_adjust");
					code.push_str(&nth_ptr_adjust.to_string());
					code.push_str(":\n");
					nth_ptr_adjust += 1;
				} else {
					code.push_str("\tsub rbx, ");
					code.push_str(&token.count.abs().to_string());
					code.push_str("\n");
					code.push_str("\tcmp rbx, 0\n");
					code.push_str("\tjge .ptr_adjust");
					code.push_str(&nth_ptr_adjust.to_string());
					code.push_str("\n");
					code.push_str("\tadd rbx, CELLS\n");
					code.push_str(".ptr_adjust");
					code.push_str(&nth_ptr_adjust.to_string());
					code.push_str(":\n");
					nth_ptr_adjust += 1;
				}
			}
			TokenType::Input => {
				code.push_str("\tmov rax, 0\n");
				code.push_str("\tmov rdi, 0\n");
				code.push_str("\tmov rsi, cells\n");
				code.push_str("\tadd rsi, rbx\n");
				code.push_str("\tmov rdx, 1\n");
				code.push_str("\tsyscall\n");
			}
			TokenType::Output => {
				code.push_str("\tmov rax, 1\n");
				code.push_str("\tmov rdi, 1\n");
				code.push_str("\tmov rsi, cells\n");
				code.push_str("\tadd rsi, rbx\n");
				code.push_str("\tmov rdx, 1\n");

				for _ in 0..token.count {
					code.push_str("\tsyscall\n");
				}
			}
			TokenType::LoopIn => {
				code.push_str(".loop");
				code.push_str(&nth_loop.to_string());
				code.push_str(":\n");
				code.push_str("\tcmp byte rbx[cells], 0\n");
				code.push_str("\tjz .loop_end");
				code.push_str(&nth_loop.to_string());
				code.push_str("\n");
				loop_stack.push(nth_loop);
				nth_loop += 1;
			}
			TokenType::LoopOut => {
				if loop_stack.len() == 0 {
					panic!("Unmatched loop out");
				}

				let loop_start = loop_stack.pop().unwrap();
				code.push_str("\tjmp .loop");
				code.push_str(&loop_start.to_string());
				code.push_str("\n");
				code.push_str(".loop_end");
				code.push_str(&loop_start.to_string());
				code.push_str(":\n");
			}
		}
	}

	code.push_str("\n");
	code.push_str("\tmov rax, 60\n");
	code.push_str("\tmov rdi, 0\n");
	code.push_str("\tsyscall\n\n");
	code.push_str("section .data\n");
	code.push_str("\tCELLS equ ");
	code.push_str(&CELLS.to_string());
	code.push_str("\n\n");
	code.push_str("section .bss\n");
	code.push_str("\tcells resb CELLS");

	return code;
}