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

    // Delimiters
    EOF,
    SEMICOLON,
	LBRACE,
	RBRACE,

    // Operators
    ASSIGN,

    // Keywords
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
}

pub type Keywords = HashMap<&'static str, TokenKind>;

pub fn get_keywords() -> Keywords {
    let mut keywords: Keywords = HashMap::with_capacity(5);

    keywords.insert("let", TokenKind::LET);
    keywords.insert("true", TokenKind::TRUE);
    keywords.insert("false", TokenKind::FALSE);
    keywords.insert("if", TokenKind::IF);
    keywords.insert("else", TokenKind::ELSE);

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
    pub value: String
}