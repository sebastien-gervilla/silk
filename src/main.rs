use silk::{
    lexer::Lexer, 
    parser::{
        parse_file, 
        Parser
    }, 
    token::TokenKind
};

fn main() {

    let code = "let x = 3;".to_string();

    let mut lexer = Lexer::new(&code);
    let mut parser = Parser::new(&mut lexer);

    parse_file(&mut parser);
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