mod codegen;
mod tokenizer;

use std::env;
use std::fs;
use std::path::Path;

static mut CELLS: u32 = 30000;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() <= 1 {
		println!("Usage: {} [options] <file>\n", args[0]);
		println!("Options:");
		println!("\t-c <cells>\t\tSpecify number of cells (default: {})", unsafe { CELLS });
		println!("\t-o <output>\t\tSpecify output file");
		return;
	}

	let file = args.last().unwrap().to_string();
	let mut output = generate_output(file.clone());

	for i in 1..args.len() - 1 {
		let arg = args[i].clone();

		if arg == "-o" {
			output = args[i + 1].clone();
		} else if arg == "-c" {
			let cells = args[i + 1].clone();

			// check if cells is a number
			if cells.parse::<u32>().is_err() {
				println!("Error with parameter -c: {} is not a number, ignoring parameter", cells);
				continue;
			}

			unsafe {
				CELLS = cells.parse::<u32>().unwrap();
			}
		}
	}

	if !Path::new(&file).exists() {
		println!("File {} does not exist", file);
		return;
	}

	let stream = fs::read_to_string(file).expect("Error reading file");

	let tokens = tokenizer::tokenize(&stream);
	let tokens = tokenizer::optimize(tokens);

	let code = codegen::generate_assembly(tokens);

	fs::write(output, code).expect("Error writing file");
}

fn generate_output(file: String) -> String {
	let output_array = file.split(".").collect::<Vec<&str>>();
	let output_array = &output_array[..output_array.len() - 1];
	let output = output_array.join(".") + ".asm";

	output
}

pub fn get_cells() -> u32 {
	unsafe {
		CELLS
	}
}