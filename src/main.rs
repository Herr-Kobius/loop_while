
use core::panic;
use std::fs;
use std::env;

mod tokenizer;
use tokenizer::Tokenizer;

mod values;
use values::Values;

mod vm;
use vm::VM;

pub mod token;

mod syntax;

pub fn read_file(file_path: &String) -> String  {
    fs::read_to_string(file_path).expect(format!("Could't read file from {}", file_path).as_str())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut source_file_contents:String;
    
    if args.len() > 1 { 
        source_file_contents = read_file(&args[1].clone());  
    }
    else {
        panic!("Provide source file path");
    }

    if !(args[1].ends_with(".loop") | args[1].ends_with(".while")) {
        panic!("Unknown file ending");
    }

    let values = Values::new(args.clone());

    println!("Inputs: {:?}", values);
    
    source_file_contents = source_file_contents.replace("\n", "").replace(" ", "");

    let mut t = Tokenizer::new(source_file_contents);
    let program_tokens = t.tokenize();

    if program_tokens.contains(&token::Token::Keyword(token::Keyword::WHILE)) {
        if !args[1].ends_with(".while") {
            panic!("File ending dose not match content");
        }
    }
    let mut vm = VM::new(values);
    
    vm.execute_program(program_tokens, false);

    println!("Output: {:?}", vm.get_result());
}