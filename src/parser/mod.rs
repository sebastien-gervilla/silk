pub mod tests;

use crate::{
    ast, 
    lexer::Lexer, 
    token::{Token, TokenKind}
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
            errors: vec![]
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn assert_peek(&mut self, expected: TokenKind) -> bool {
        self.next_token();
        if self.current_token.kind == expected {
            return true
        } else {
            self.add_error(
                String::from(
                    format!(
                        "Expected token {:?}, instead got: {:?}", 
                        expected, 
                        self.current_token.kind
                    )
                )
            );
            return false
        }
    }

    pub fn get_current_token(&self) -> Token {
        self.current_token.clone()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

}

pub fn parse_file(parser: &mut Parser) -> ast::File {
    let mut file = ast::File {
        node: ast::Node {
            token: parser.get_current_token()
        },
        statements: vec![]
    };

    while parser.peek_token.kind != TokenKind::EOF {
        if let Some(statement) = parse_statement(parser) {
            file.statements.push(statement);
        }

        parser.next_token();
    }

    return file
}

fn parse_statement(parser: &mut Parser) -> Option<ast::Statement> {
    match parser.current_token.kind {
        TokenKind::LET => Some(parse_let_stament(parser)),
        _ => {
            parser.add_error(
                String::from(
                    format!("Unsupported statement {:?}", parser.current_token.value)
                )
            );

            None
        }
    }
}

fn parse_let_stament(parser: &mut Parser) -> ast::Statement {

    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.assert_peek(TokenKind::IDENTIFIER);

    let identifier = parse_identifier(parser);

    // TODO: Allow for initialization only
    parser.assert_peek(TokenKind::ASSIGN);

    parser.next_token();
    let expression = parse_expression(parser);

    parser.assert_peek(TokenKind::SEMICOLON);

    ast::Statement::Let(
        ast::LetStatement {
            node,
            identifier,
            expression: Some(expression)
        }
    )
}

fn parse_identifier(parser: &Parser) -> ast::Identifier {
    ast::Identifier {
        value: parser.current_token.value.clone()
    }
}

fn parse_expression(parser: &Parser) -> ast::Expression {
    ast::Expression {  }
}