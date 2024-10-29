#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenKind};

    fn test_lex(code: &String, expected_tokens: &Vec<TokenKind>) {
        let mut lexer = Lexer::new(&code);

        let mut index = 0;
        let mut token = lexer.next_token();
        while token.kind != TokenKind::EOF {
            assert_eq!(token.kind, expected_tokens[index]);
            index += 1;
            token = lexer.next_token();
        }
    }

    #[test]
    fn test_read_comments() {
        let code = "// Comment \n\n //Other comment \n ".to_string();
        let expected_tokens = vec![
            TokenKind::LET,
        ];

        test_lex(&code, &expected_tokens);
    }

    #[test]
    fn test_read_numbers() {
        let code = "3 45 0".to_string();
        let expected_tokens = vec![
            TokenKind::NUMBER,
            TokenKind::NUMBER,
            TokenKind::NUMBER,
        ];

        test_lex(&code, &expected_tokens);
    }

    #[test]
    fn test_read_characters() {
        let code = "'''".to_string();
        let expected_tokens = vec![
            TokenKind::CHARACTER,
        ];
        test_lex(&code, &expected_tokens);

        let code = "'b' != 'c'".to_string();
        let expected_tokens = vec![
            TokenKind::CHARACTER,
            TokenKind::NOT_EQUALS,
            TokenKind::CHARACTER,
        ];
        test_lex(&code, &expected_tokens);
    }

    #[test]
    fn test_read_identifier() {
        let code = "aBc x_y_z _m".to_string();
        let expected_tokens = vec![
            TokenKind::IDENTIFIER,
            TokenKind::IDENTIFIER,
            TokenKind::IDENTIFIER,
        ];

        test_lex(&code, &expected_tokens);
    }

    #[test]
    fn test_read_keywords() {
        let code = "let".to_string();
        let expected_tokens = vec![
            TokenKind::LET,
        ];

        test_lex(&code, &expected_tokens);
    }

    #[test]
    fn test_read_doublecolon() {
        let code = "::".to_string();
        let expected_tokens = vec![
            TokenKind::DOUBLECOLON,
        ];
        test_lex(&code, &expected_tokens);

        let code = "id::id".to_string();
        let expected_tokens = vec![
            TokenKind::IDENTIFIER,
            TokenKind::DOUBLECOLON,
            TokenKind::IDENTIFIER,
        ];
        test_lex(&code, &expected_tokens);
    }
}