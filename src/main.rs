use std::{env, fs};

use silk::backend::{
    bytecode::Chunk,
    object::FunctionObject,
    compiler::Compiler,
    vm::VM,
};

use silk::frontend::{
    lexer::Lexer, 
    parser::{
        parse_file, 
        Parser
    }, 
    token::TokenKind, 
    typecheck::check_program,
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

    let mut lexer = Lexer::new(&code);
    let mut parser = Parser::new(&mut lexer);

    let ast = parse_file(&mut parser);
    if parser.errors.len() > 0 {
        for error in parser.errors {
            println!("{error}");
        }

        panic!("Found parsing errors.");
    }
    println!("Parsing completed.");

    check_program(&ast);
    println!("Typechecking completed.");

    let function = &mut FunctionObject {
        chunk: Chunk::new(),
        arity: 0,
        name: String::from("Global"),
    };

    let mut compiler = Compiler::new(function);
    let function = compiler.compile(&ast);

    let mut vm = VM::new(function);
    vm.run();
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