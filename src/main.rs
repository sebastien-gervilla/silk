use silk::{
    compiler::{bytecode::Chunk, vm::VM}, lexer::Lexer, token::TokenKind
};

fn main() {

    let code = "3 + 3;";

    let mut chunk = Chunk::new();
    let mut vm = VM::new(&mut chunk);
    vm.interpret(code);
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