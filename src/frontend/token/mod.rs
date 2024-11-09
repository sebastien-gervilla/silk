use std::collections::HashMap;

// Tokens
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Misc
    UNKNOW,

    // Literals
    IDENTIFIER,
    NUMBER,
    STRING,
    CHARACTER,

    // Operators
    NOT,
	EQUALS,
	NOT_EQUALS,
	GREATER_THAN,
	LESS_THAN,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    AND,
    OR,

    // Delimiters
    EOF,
    COMMA,
    COLON,
    DOUBLECOLON,
    SEMICOLON,
	LPAREN,
	RPAREN,
	LBRACE,
	RBRACE,
	LBRACKET,
	RBRACKET,

    // Operators
    ASSIGN,

    // Keywords
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    WHILE,
    BREAK,
    FUNCTION,
    RETURN,

    // Types
    PRIMITIVE_TYPE,
}

pub type Keywords = HashMap<&'static str, TokenKind>;

pub fn get_keywords() -> Keywords {
    let mut keywords: Keywords = HashMap::with_capacity(11);

    keywords.insert("let", TokenKind::LET);
    keywords.insert("true", TokenKind::TRUE);
    keywords.insert("false", TokenKind::FALSE);
    keywords.insert("if", TokenKind::IF);
    keywords.insert("else", TokenKind::ELSE);
    keywords.insert("while", TokenKind::WHILE);
    keywords.insert("break", TokenKind::BREAK);
    keywords.insert("fn", TokenKind::FUNCTION);
    keywords.insert("return", TokenKind::RETURN);

    keywords.insert("int", TokenKind::PRIMITIVE_TYPE);
    keywords.insert("bool", TokenKind::PRIMITIVE_TYPE);
    keywords.insert("void", TokenKind::PRIMITIVE_TYPE);

    return keywords
}

pub fn get_token_kind(keywords: &Keywords, identifier: &str) -> TokenKind {
    match keywords.get(identifier) {
        Some(keyword) => keyword.clone(),
        None => TokenKind::IDENTIFIER
    }
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub column: usize,
}