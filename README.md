# Brainfuck Compiler

Compiler from Brainfuck to NASM Linux x86-64 Assembly

# How to build

Use `cargo build` at the root of the project

# Usage

The main command is `brainfuck_compiler [options] <file>`

## Options

`-c <cells>`: Specify number of cells (default: 30000)

`-o <output>`: Specify output file