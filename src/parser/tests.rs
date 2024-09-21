#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer, 
        parser::{
            parse_file, 
            Parser
        }
    };

    fn test_parse(code: &String) {
        let mut lexer = Lexer::new(code);
        let mut parser = Parser::new(&mut lexer);

        parse_file(&mut parser);

        if parser.errors.len() > 0 {
            for error in parser.errors {
                println!("{error}");
            }

            panic!("Found parsing errors.");
        }
    }

    #[test]
    fn test_parse_let_statement() {
        let code = String::from("let x = 10;");

        test_parse(&code);
    }

    #[test]
    fn test_parse_string_literal() {
        let code = String::from("let x = \"aD_67'le\";");

        test_parse(&code);
    }

}