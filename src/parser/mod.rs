pub mod tests;

use std::collections::HashMap;

use crate::{
    ast,
    lexer::Lexer,
    token::{Token, TokenKind}, typecheck::types::Type
};

// Precedences
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
	ASSIGNMENT,     // =
	OR,             // ||
	AND,            // &&
	EQUALITY,       // ==, !=
	LESSGREATER,    // >, <
	SUM,            // +, -
	PRODUCT,        // *, /
	PREFIX,         // -expression, !expression
	ACCESS,         // expression::expression
	CALL,           // identifier(expression, expression)
}

type Precedences = HashMap<TokenKind, Precedence>;

fn get_precedences() -> Precedences {
    let mut precedences = Precedences::with_capacity(12);

    precedences.insert(TokenKind::ASSIGN, Precedence::ASSIGNMENT);
    precedences.insert(TokenKind::OR, Precedence::OR);
    precedences.insert(TokenKind::AND, Precedence::AND);
    precedences.insert(TokenKind::EQUALS, Precedence::EQUALITY);
    precedences.insert(TokenKind::NOT_EQUALS, Precedence::EQUALITY);
    precedences.insert(TokenKind::GREATER_THAN, Precedence::LESSGREATER);
    precedences.insert(TokenKind::LESS_THAN, Precedence::LESSGREATER);
    precedences.insert(TokenKind::PLUS, Precedence::SUM);
    precedences.insert(TokenKind::MINUS, Precedence::SUM);
    precedences.insert(TokenKind::ASTERISK, Precedence::PRODUCT);
    precedences.insert(TokenKind::SLASH, Precedence::PRODUCT);
    precedences.insert(TokenKind::DOUBLECOLON, Precedence::ACCESS);
    precedences.insert(TokenKind::LPAREN, Precedence::CALL);

    precedences
}

type PrefixParsingFunction = fn(&mut Parser) -> Box<ast::Expression>;
type InfixParsingFunction = fn(&mut Parser, Box<ast::Expression>) -> Box<ast::Expression>;

type PrefixParsingFunctions = HashMap<TokenKind, PrefixParsingFunction>;
type InfixParsingFunctions = HashMap<TokenKind, InfixParsingFunction>;

fn get_prefix_parsing_functions() -> PrefixParsingFunctions {
    let mut functions: PrefixParsingFunctions = HashMap::with_capacity(11);

    functions.insert(TokenKind::IDENTIFIER, parse_identifier_expression);
    functions.insert(TokenKind::NUMBER, parse_number_literal);
    functions.insert(TokenKind::STRING, parse_string_literal);
    functions.insert(TokenKind::TRUE, parse_boolean_literal);
    functions.insert(TokenKind::FALSE, parse_boolean_literal);

    functions.insert(TokenKind::FUNCTION, parse_function);
    functions.insert(TokenKind::RETURN, parse_return_expression);

    functions.insert(TokenKind::LPAREN, parse_grouped_expression);
    functions.insert(TokenKind::LBRACE, parse_block_expression);
    functions.insert(TokenKind::IF, parse_if_expression);
    functions.insert(TokenKind::WHILE, parse_while_expression);

    functions.insert(TokenKind::NOT, parse_prefix_expression);
    functions.insert(TokenKind::MINUS, parse_prefix_expression);

    return functions
}

fn get_infix_parsing_functions() -> InfixParsingFunctions {
    let mut functions: InfixParsingFunctions = HashMap::with_capacity(12);

    functions.insert(TokenKind::PLUS, parse_infix_expression);
    functions.insert(TokenKind::MINUS, parse_infix_expression);
    functions.insert(TokenKind::ASTERISK, parse_infix_expression);
    functions.insert(TokenKind::SLASH, parse_infix_expression);
    functions.insert(TokenKind::EQUALS, parse_infix_expression);
    functions.insert(TokenKind::NOT_EQUALS, parse_infix_expression);
    functions.insert(TokenKind::GREATER_THAN, parse_infix_expression);
    functions.insert(TokenKind::LESS_THAN, parse_infix_expression);
    functions.insert(TokenKind::AND, parse_infix_expression);
    functions.insert(TokenKind::OR, parse_infix_expression);

    functions.insert(TokenKind::ASSIGN, parse_assignment_expression);
    functions.insert(TokenKind::DOUBLECOLON, parse_access_expression);
    
    functions.insert(TokenKind::LPAREN, parse_call_expression);

    return functions
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
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
        if self.peek_token.kind == expected {
            self.next_token();
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

    pub fn is_current_token(&self, token_kind: TokenKind) -> bool {
        self.current_token.kind == token_kind
    }

    pub fn get_peek_precedence(&self) -> Precedence {
        match self.precedences.get(&self.peek_token.kind) {
            Some(precedence) => precedence.clone(),
            None => Precedence::LOWEST,
        }
    }

    pub fn get_current_precedence(&self) -> Precedence {
        match self.precedences.get(&self.current_token.kind) {
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
        file.statements.push(parse_statement(parser));
        parser.next_token();
    }

    return file
}

// Statements

fn parse_statement(parser: &mut Parser) -> ast::Statement {
    match parser.current_token.kind {
        TokenKind::LET => parse_let_stament(parser),
        _ => parse_expression_statement(parser)
    }
}

fn parse_let_stament(parser: &mut Parser) -> ast::Statement {

    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.assert_peek(TokenKind::IDENTIFIER);

    let identifier = parse_identifier(parser);

    let mut annotation : Option<Type> = None;

    // Parsing annotation
    if parser.is_peek_token(TokenKind::COLON) {
        parser.assert_peek(TokenKind::COLON);
        parser.assert_peek(TokenKind::ANNOTATION);
        annotation = Some(parse_type(parser));
    }

    // TODO: Allow for initialization only
    parser.assert_peek(TokenKind::ASSIGN);

    parser.next_token();
    let expression = parse_expression(parser, Precedence::LOWEST);

    parser.assert_peek(TokenKind::SEMICOLON);

    ast::Statement::Let(
        ast::LetStatement {
            node,
            identifier,
            annotation,
            expression: Some(expression)
        }
    )
}

fn parse_expression_statement(parser: &mut Parser) -> ast::Statement {
    let expression = ast::Statement::Expression(
        ast::ExpressionStatement {
            node: ast::Node {
                token: parser.get_current_token()
            },
            expression: parse_expression(parser, Precedence::LOWEST)
        }
    );

    parser.assert_peek(TokenKind::SEMICOLON);

    return expression
}

// Expressions

fn parse_expression(parser: &mut Parser, precedence: Precedence) -> Box<ast::Expression> {
    let prefix_function = match parser.prefix_parsing_functions.get(&parser.current_token.kind) {
        Some(prefix_function) => *prefix_function,
        None => {
            panic!("Prefix function for token {:?} not found", parser.current_token.kind)
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

fn parse_identifier_expression(parser: &mut Parser) -> Box<ast::Expression> {
    Box::new(
        ast::Expression::Identifier(
            parse_identifier(parser)
        )
    )
}

fn parse_identifier(parser: &mut Parser) -> ast::Identifier {
    ast::Identifier {
        node: ast::Node {
            token: parser.get_current_token(),
        },
        value: parser.current_token.value.clone()
    }
}

fn parse_number_literal(parser: &mut Parser) -> Box<ast::Expression> {
    let value = match parser.current_token.value.parse::<isize>() {
        Ok(value) => value,
        Err(error) => {
            parser.add_error(error.to_string());
            0 // TODO: We want to handle this differently later on
        },
    };

    Box::new(
        ast::Expression::NumberLiteral(
            ast::NumberLiteral {
                node: ast::Node {
                    token: parser.get_current_token(),
                },
                value
            }
        )
    )
}

fn parse_string_literal(parser: &mut Parser) -> Box<ast::Expression> {
    Box::new(
        ast::Expression::StringLiteral(
            ast::StringLiteral {
                node: ast::Node {
                    token: parser.get_current_token(),
                },
                value: parser.current_token.value.clone()
            }
        )
    )
}

fn parse_boolean_literal(parser: &mut Parser) -> Box<ast::Expression> {
    Box::new(
        ast::Expression::BooleanLiteral(
            ast::BooleanLiteral {
                node: ast::Node {
                    token: parser.get_current_token(),
                },
                value: parser.current_token.value == "true"
            }
        )
    )
}

fn parse_function(parser: &mut Parser) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.assert_peek(TokenKind::IDENTIFIER);

    let identifier = parse_identifier(parser);

    parser.assert_peek(TokenKind::LPAREN);

    let parameters = parse_function_parameters(parser);

    parser.assert_peek(TokenKind::RPAREN);

    parser.assert_peek(TokenKind::MINUS);
    parser.assert_peek(TokenKind::GREATER_THAN);
    parser.assert_peek(TokenKind::ANNOTATION);

    let annotation = parse_type(parser);

    parser.assert_peek(TokenKind::LBRACE);
    
    let body = parse_block_expression(parser);

    return Box::new(
        ast::Expression::Function(
            ast::Function {
                node,
                identifier,
                parameters,
                annotation,
                body
            }
        )
    )
}

fn parse_return_expression(parser: &mut Parser) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.next_token();

    return Box::new(
        ast::Expression::Return(
            ast::ReturnExpression {
                node,
                expression: parse_expression(parser, Precedence::LOWEST),
            }
        )
    )
}

fn parse_function_parameters(parser: &mut Parser) -> Vec<ast::FunctionParameter> {
    let mut parameters = Vec::<ast::FunctionParameter>::new();

    if parser.is_peek_token(TokenKind::RPAREN) {
        return parameters
    }

    parser.assert_peek(TokenKind::IDENTIFIER);
    let identifier = parse_identifier(parser);
    
    parser.assert_peek(TokenKind::COLON);
    parser.assert_peek(TokenKind::ANNOTATION);

    parameters.push(
        ast::FunctionParameter {
            identifier,
            annotation: parse_type(parser)
        }
    );

    
    while !parser.is_peek_token(TokenKind::RPAREN) {
        parser.assert_peek(TokenKind::COMMA);
        parser.assert_peek(TokenKind::IDENTIFIER);
        let identifier = parse_identifier(parser);

        parser.assert_peek(TokenKind::COLON);
        parser.assert_peek(TokenKind::ANNOTATION);
        parameters.push(
            ast::FunctionParameter {
                identifier,
                annotation: parse_type(parser)
            }
        );
    }

    return parameters
}

fn parse_prefix_expression(parser: &mut Parser) -> Box<ast::Expression> {

    let node = ast::Node {
        token: parser.get_current_token()
    };

    let operator = parser.current_token.value.clone();

    parser.next_token();

    Box::new(
        ast::Expression::Prefix(
            ast::PrefixExpression {
                node,
                operator,
                expression: parse_expression(parser, Precedence::PREFIX)
            }
        )
    )
}

fn parse_infix_expression(parser: &mut Parser, left_expression: Box<ast::Expression>) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    let precedence = parser.get_current_precedence();
    let operator = parser.current_token.value.clone();

    parser.next_token();

    Box::new(
        ast::Expression::Infix(
            ast::InfixExpression {
                node,
                operator,
                left_expression,
                right_expression: parse_expression(parser, precedence)
            }
        )
    )
}

fn parse_grouped_expression(parser: &mut Parser) -> Box<ast::Expression> {
    parser.next_token();

    let expression = parse_expression(parser, Precedence::LOWEST);

    parser.assert_peek(TokenKind::RPAREN);

    return expression
}

fn parse_assignment_expression(parser: &mut Parser, expression: Box<ast::Expression>) -> Box<ast::Expression> {

    let identifier = match *expression {
        ast::Expression::Identifier(identifier) => identifier,
        _ => {
            parser.add_error(String::from("Expected identifier."));
            panic!("Expected identifier.");
        },
    };

    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.next_token();

    return Box::new(
        ast::Expression::Assign(
            ast::AssignmentExpression {
                node,
                identifier,
                expression: parse_expression(parser, Precedence::LOWEST),
            }
        )
    )
}

fn parse_block_expression(parser: &mut Parser) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.next_token();

    let mut statements = Vec::<ast::Statement>::new();
    while !parser.is_current_token(TokenKind::RBRACE) {
        statements.push(parse_statement(parser));
        parser.next_token();
    }

    return Box::new(
        ast::Expression::Block(
            ast::BlockExpression {
                node,
                statements
            }
        )
    )
}

fn parse_if_expression(parser: &mut Parser) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.next_token();

    let condition = parse_expression(parser, Precedence::LOWEST);

    parser.assert_peek(TokenKind::LBRACE);

    let consequence = parse_expression(parser, Precedence::LOWEST);

    let mut if_expression = ast::IfExpression {
        node,
        condition,
        consequence,
        alternative: None
    };

    if !parser.is_peek_token(TokenKind::ELSE) {
        return Box::new(ast::Expression::If(if_expression))
    };

    // Parsing else expression
    parser.next_token();

    if parser.is_peek_token(TokenKind::IF) {
        parser.next_token();
        if_expression.alternative = Some(parse_expression(parser, Precedence::LOWEST));
        return Box::new(ast::Expression::If(if_expression))
    }

    parser.assert_peek(TokenKind::LBRACE);
    if_expression.alternative = Some(parse_expression(parser, Precedence::LOWEST));
    return Box::new(ast::Expression::If(if_expression))
}

fn parse_while_expression(parser: &mut Parser) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.next_token();
    let condition = parse_expression(parser, Precedence::LOWEST);


    parser.assert_peek(TokenKind::LBRACE);

    let iteration = parse_expression(parser, Precedence::LOWEST);

    return Box::new(
        ast::Expression::While(
            ast::WhileExpression {
                node,
                condition,
                iteration
            }
        )
    )
}

fn parse_call_expression(parser: &mut Parser, identifier: Box<ast::Expression>) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    let arguments = parse_call_arguments(parser);

    parser.assert_peek(TokenKind::RPAREN);

    return Box::new(
        ast::Expression::Call(
            ast::CallExpression {
                node,
                identifier,
                arguments,
            }
        )
    )
}

fn parse_call_arguments(parser: &mut Parser) -> Vec<Box<ast::Expression>> {
    let mut arguments = Vec::<Box<ast::Expression>>::new();

    if parser.is_peek_token(TokenKind::RPAREN) {
        return arguments
    }

    parser.next_token();
    arguments.push(parse_expression(parser, Precedence::LOWEST));

    while !parser.is_peek_token(TokenKind::RPAREN) {
        parser.assert_peek(TokenKind::COMMA);
        parser.next_token();
        arguments.push(parse_expression(parser, Precedence::LOWEST));
    }
    
    return arguments
}

fn parse_access_expression(parser: &mut Parser, left_expression: Box<ast::Expression>) -> Box<ast::Expression> {
    let node = ast::Node {
        token: parser.get_current_token()
    };

    parser.next_token();
    let right_expression = parse_expression(parser, Precedence::LOWEST);

    return Box::new(
        ast::Expression::Access(
            ast::AccessExpression {
                node,
                left_expression,
                right_expression,
            }
        )
    )
}

// Types

fn parse_type(parser: &mut Parser) -> Type {
    match parser.current_token.value.as_str() {
        "int" => Type::Integer,
        "bool" => Type::Boolean,
        _ => {
            parser.add_error(format!("Invalid type '{}'", parser.current_token.value));
            Type::Integer
        },
    }
}