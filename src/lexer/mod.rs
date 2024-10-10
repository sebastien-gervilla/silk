pub mod tests;

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
    line: usize,
    column: usize,
    position: usize,
    peek_position: usize,
    keywords: Keywords
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        let mut lexer = Self {
            code: code.as_bytes(),
            character: 0,
            line: 0,
            column: 0,
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
            value: self.u8_to_string(self.character),
            line: self.line,
            column: self.column
        };

        match self.character {
            b'=' => {
                if self.get_next_character() == b'=' {
                    self.next_character();
                    token.kind = TokenKind::EQUALS;
                    token.value = String::from("==");
                } else {
                    token.kind = TokenKind::ASSIGN;
                }
            },
            b';' => token.kind = TokenKind::SEMICOLON,
            b'"' => {
                token.kind = TokenKind::STRING;
                token.value = self.read_string();
            },
            b'!' => {
                if self.get_next_character() == b'=' {
                    self.next_character();
                    token.kind = TokenKind::NOT_EQUALS;
                    token.value = String::from("!=");
                } else {
                    token.kind = TokenKind::NOT;
                }
            },
            b'>' => token.kind = TokenKind::GREATER_THAN,
            b'<' => token.kind = TokenKind::LESS_THAN,
            b'+' => token.kind = TokenKind::PLUS,
            b'-' => token.kind = TokenKind::MINUS,
            b'*' => token.kind = TokenKind::ASTERISK,
            b'/' => token.kind = TokenKind::SLASH,
            b'{' => token.kind = TokenKind::LBRACE,
            b'}' => token.kind = TokenKind::RBRACE,
            b'(' => token.kind = TokenKind::LPAREN,
            b')' => token.kind = TokenKind::RPAREN,
            b',' => token.kind = TokenKind::COMMA,
            b':' => token.kind = TokenKind::COLON,
            b'&' => {
                if self.get_next_character() == b'&' {
                    self.next_character();
                    token.kind = TokenKind::AND;
                    token.value = String::from("&&");
                }
            },
            b'|' => {
                if self.get_next_character() == b'|' {
                    self.next_character();
                    token.kind = TokenKind::OR;
                    token.value = String::from("||");
                }
            },
            _ => {
                if self.is_valid_character() {
                    token.value = self.read_identifier();
                    token.kind = get_token_kind(&self.keywords, &token.value);
                    return token
                } else if self.is_digit() {
                    token.value = self.read_number();
                    token.kind = TokenKind::NUMBER;
                    return token
                } else if self.character == 0 {
                    token.kind = TokenKind::EOF;
                }
            }
        }

        self.next_character();

        return token
    }

    fn next_character(&mut self) {
        if self.peek_position >= self.code.len() {
            self.position = self.code.len();
            self.character = 0;
            return;
        }

        self.character = self.code[self.peek_position];
        self.position = self.peek_position;
        self.peek_position += 1;
    }

    fn get_next_character(&mut self) -> u8 {
        if self.peek_position >= self.code.len() {
            return 0
        }

        return self.code[self.peek_position]
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.character {
                b' ' | b'\t' | b'\r' => self.next_character(),
                b'\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.next_character();
                },
                b'/' => {
                    if self.get_next_character() != b'/' {
                        return;
                    }

                    while self.character != b'\n' {
                        self.next_character();

                        if self.character == 0 {
                            return;
                        }
                    }

                    self.next_character();
                },
                _ => return
            }
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

    fn read_string(&mut self) -> String {
        let initial_position = self.position + 1;

        self.next_character();
        while self.character != b'"' {
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