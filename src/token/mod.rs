use std::collections::HashMap;

// Tokens
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Misc
    UNKNOW,

    // Literals
    IDENTIFIER,
    NUMBER,

    // Delimiters
    EOF,
    SEMICOLON,

    // Operators
    ASSIGN,

    // Keywords
    LET
}

pub type Keywords = HashMap<&'static str, TokenKind>;

pub fn get_keywords() -> Keywords {
    let mut keywords: Keywords = HashMap::with_capacity(1);

    keywords.insert("let", TokenKind::LET);

    return keywords
}

pub fn get_token_kind(keywords: &Keywords, identifier: &str) -> TokenKind {
    match keywords.get(identifier) {
        Some(keyword) => keyword.clone(),
        None => TokenKind::IDENTIFIER
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String
}