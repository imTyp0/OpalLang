use std::{fs, env, process::{exit, Command}};

mod tokenization;
mod generation;
mod parser;

const NASM_ARGS: [&str; 4] = ["-felf64", "build/out.asm", "-o", "build/out.o"];
const LD_ARGS: [&str; 3] = ["build/out.o", "-o", "build/test"];

fn main() {
	let argv: Vec<String> = env::args().collect();
	
	// Make sure we have correct number of args
	if argv.len() != 2{
		println!("[!] Incorrect usage. Correct usage is:\nrefine `inputFile.op`");
		exit(1);
	}

    // Make sure extension matches
    if argv[1][argv[1].find('.').unwrap()..] != *".op"{
        println!("[!] Incorrect source file extension. Extension should be `.op`");
        exit(1);
    }

	// Contents of the input file
	let contents = fs::read_to_string(&argv[1]);
	// Make sure the file exists
    if contents.is_err(){
        eprintln!("[!] Couldn't read input file OR input file does not exist.");
        exit(1);
    }
	// Turn the contents of the input file into a list of tokens
	let tokens = tokenization::tokenize(contents.unwrap());

	// Parse the tokens into nodes
	let tree = parser::parse_prog(tokens);

	if tree.is_none(){
		println!("Invalid program");
		exit(1);
	}

	// Turn the nodes into x86 64 ASM
	let asm = generation::gen_prog(tree.unwrap());

	// Write the assembly out to a file, assemble it, and link it
	fs::write("build/out.asm", asm).expect("Couldn't write assembly file.");
	Command::new("nasm")
		.args(NASM_ARGS)
		.output()
		.expect("nasm couldn't assemble the file");
	Command::new("ld")
		.args(LD_ARGS)
		.output()
		.expect("ld couldn't link object file");
}
