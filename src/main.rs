use silk::{
    lexer::Lexer, 
    token::TokenKind
};

fn main() {

    let code = "let x = 3;".to_string();

    let mut lexer = Lexer::new(&code);

    let mut token = lexer.next_token();
    println!("Starting lexing...");
    while token.kind != TokenKind::EOF {
        token = lexer.next_token();
        println!("Token {:?}: {}", token.kind, token.value);
    }
}