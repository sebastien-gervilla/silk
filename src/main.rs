use std::{env, fs};

use silk::{
    compiler::{bytecode::Chunk, vm::VM}, lexer::Lexer, token::TokenKind
};

fn main() {

    let current_directory = match env::current_dir() {
        Ok(path) => {
            println!("\n\n=> Directory: {:?}", path.as_path());
            path
        },
        Err(error) => panic!("Couldn't get current working directory : {error}"),
    };

    let code_path = current_directory.join("tests/input.silk");

    let code = match fs::read_to_string(code_path) {
        Ok(code) => code,
        Err(error) => panic!("Couldn't read code file : {error}"),
    };

    let mut chunk = Chunk::new();
    let mut vm = VM::new(&mut chunk);
    vm.interpret(&code);
}

#[allow(dead_code)]
fn print_lexed_code(code: &String) {
    let mut lexer = Lexer::new(code);

    let mut token = lexer.next_token();
    println!("Starting lexing...");
    while token.kind != TokenKind::EOF {
        println!("Token {:?}: {}", token.kind, token.value);
        token = lexer.next_token();
    }
}