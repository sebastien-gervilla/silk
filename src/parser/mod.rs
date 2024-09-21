pub mod tests;

use std::collections::HashMap;

use crate::{
    ast, 
    lexer::Lexer, 
    token::{Token, TokenKind}
};

// Precedences
#[derive(Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
}

type Precedences = HashMap<TokenKind, Precedence>;

fn get_precedences() -> Precedences {
    let mut precedences = Precedences::with_capacity(1);

    precedences
}

type PrefixParsingFunction = fn(&mut Parser) -> ast::Expression;
type InfixParsingFunction = fn(&mut Parser, ast::Expression) -> ast::Expression;

type PrefixParsingFunctions = HashMap<TokenKind, PrefixParsingFunction>;
type InfixParsingFunctions = HashMap<TokenKind, InfixParsingFunction>;

fn get_prefix_parsing_functions() -> PrefixParsingFunctions {
    let mut functions: PrefixParsingFunctions = HashMap::with_capacity(2);

    functions.insert(TokenKind::IDENTIFIER, parse_identifier);
    functions.insert(TokenKind::NUMBER, parse_number_literal);

    return functions
}

fn get_infix_parsing_functions() -> InfixParsingFunctions {
    let mut functions: InfixParsingFunctions = HashMap::with_capacity(0);

    return functions
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parsing_functions: PrefixParsingFunctions,
    infix_parsing_functions: InfixParsingFunctions,
    precedences: Precedences
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
            prefix_parsing_functions: get_prefix_parsing_functions(),
            infix_parsing_functions: get_infix_parsing_functions(),
            precedences: get_precedences()
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

    pub fn is_peek_token(&self, token_kind: TokenKind) -> bool {
        self.peek_token.kind == token_kind
    }

    pub fn get_peek_precedence(&self) -> Precedence {
        match self.precedences.get(&self.peek_token.kind) {
            Some(precedence) => precedence.clone(),
            None => Precedence::LOWEST,
        }
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

// Statements

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
    let expression = parse_expression(parser, Precedence::LOWEST);

    parser.assert_peek(TokenKind::SEMICOLON);

    ast::Statement::Let(
        ast::LetStatement {
            node,
            identifier,
            expression: Some(expression)
        }
    )
}

// Expressions

fn parse_expression(parser: &mut Parser, precedence: Precedence) -> ast::Expression {
    let prefix_function = match parser.prefix_parsing_functions.get(&parser.current_token.kind) {
        Some(prefix_function) => *prefix_function,
        None => {
            parser.add_error("".to_string());
            panic!("TODO")
        }
    };

    let mut left_expression = prefix_function(parser);
    
    while !parser.is_peek_token(TokenKind::SEMICOLON) && precedence < parser.get_peek_precedence() {
        let infix_function = match parser.infix_parsing_functions.get(&parser.peek_token.kind) {
            Some(infix_function) => *infix_function,
            None => return left_expression
        };

        parser.next_token();
        left_expression = infix_function(parser, left_expression);
    }

    return left_expression
}

fn parse_identifier(parser: &mut Parser) -> ast::Expression {
    ast::Expression::Identifier(
        ast::Identifier {
            value: parser.current_token.value.clone()
        }
    )
}

fn parse_number_literal(parser: &mut Parser) -> ast::Expression {
    let value = match parser.current_token.value.parse::<isize>() {
        Ok(value) => value,
        Err(error) => {
            parser.add_error(error.to_string());
            0 // TODO: We want to handle this differently later on
        },
    };

    ast::Expression::NumberLiteral(
        ast::NumberLiteral {
            value
        }
    )
}