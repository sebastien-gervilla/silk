use crate::token::{
    Token, // TODO: This may belong to lexer instead of token
    TokenKind,
    Keywords,
    get_keywords,
    get_token_kind
};

pub struct Lexer<'a> {
    code: &'a[u8],
    character: u8,
    position: usize,
    peek_position: usize,
    keywords: Keywords
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a String) -> Self {
        let mut lexer = Self {
            code: code.as_bytes(),
            character: 0,
            position: 0,
            peek_position: 0,
            keywords: get_keywords()
        };

        if lexer.code.len() > 0 {
            lexer.next_character();
            return lexer
        }

        panic!("Empty file.");
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut token = Token {
            kind: TokenKind::UNKNOW,
            value: self.u8_to_string(self.character)
        };
        
        if self.character == b'=' {
            token.kind = TokenKind::ASSIGN;
        } else if self.character == b';' {
            token.kind = TokenKind::SEMICOLON;
        } else {
            if self.is_valid_character() {
                token.value = self.read_identifier();
                token.kind = get_token_kind(&self.keywords, &token.value);
            } else if self.is_digit() {
                token.value = self.read_number();
                token.kind = TokenKind::NUMBER;
            } else if self.character == 0 {
                token.kind = TokenKind::EOF;
            }
        }

        self.next_character();

        return token
    }

    fn next_character(&mut self) {
        if self.peek_position >= self.code.len() {
            self.character = 0;
            return;
        }

        self.character = self.code[self.peek_position];
        self.position = self.peek_position;
        self.peek_position += 1;
    }

    fn skip_whitespace(&mut self) {
        if self.character == b' ' 
        || self.character == b'\n' 
        || self.character == b'\t' 
        || self.character == b'\r' {
            self.next_character();
        }
    }

    fn is_valid_character(&self) -> bool {
        (self.character <= b'z' && self.character >= b'a')
            || (self.character <= b'Z' && self.character >= b'A')
            || self.character == b'_'
    }

    fn is_digit(&self) -> bool {
        self.character <= b'9' && self.character >= b'0'
    }

    fn read_identifier(&mut self) -> String {
        let initial_position = self.position;
        while self.is_valid_character() {
            self.next_character();
        }

        return match String::from_utf8(self.code[initial_position..self.position].to_vec()) {
            Ok(string) => string,
            Err(error) => panic!("{error}")
        };
    }

    fn read_number(&mut self) -> String {
        let initial_position = self.position;
        while self.is_digit() {
            self.next_character();
        }

        return match String::from_utf8(self.code[initial_position..self.position].to_vec()) {
            Ok(string) => string,
            Err(error) => panic!("{error}")
        };
    }

    fn u8_to_string(&self, u8: u8) -> String {
        return match String::from_utf8([u8].to_vec()) {
            Ok(string) => string,
            Err(error) => panic!("{error}")
        };
    }

}