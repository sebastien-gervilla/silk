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
    typecheck::check_program,
};

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let current_directory = match env::current_dir() {
        Ok(path) => {
            println!("\n\n=> Directory: {:?}", path.as_path());
            path
        },
        Err(error) => panic!("Couldn't get current working directory : {error}"),
    };

    let code_path = current_directory.join(file_path);

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

    check_program(&ast);

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