mod codegen;
mod tokenizer;

use std::fs;
// use std::process::Command;

const FILE: &str = "test/test.bf";
const OUTPUT: &str = "test/test.asm";
const CELLS: u32 = 16;

fn main() {
	let stream = fs::read_to_string(FILE).expect("Error reading file");

	let tokens = tokenizer::tokenize(&stream);
	let tokens = tokenizer::optimize(tokens);

	let code = codegen::generate_assembly(tokens);

	fs::write(OUTPUT, code).expect("Error writing file");
}