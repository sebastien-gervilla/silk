#[cfg(test)]
mod tests {
    use crate::frontend::{
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
                println!("Parsing error: {error}");
            }

            panic!("Found parsing errors.");
        }
    }

    #[test]
    fn test_parse_let_statement() {
        // Initialized, with annotation
        let code = String::from("let x: int = 10;");
        test_parse(&code);

        // Initialized, without annotation
        let code = String::from("let x = 10;");
        test_parse(&code);

        // Uninitialized, with annotation
        let code = String::from("let x: int;");
        test_parse(&code);

        // Uninitialized, without annotation
        // (Semantic analysis will reject it)
        let code = String::from("let x;");
        test_parse(&code);
    }

    #[test]
    fn test_parse_character_literal() {
        // Normal character
        let code = String::from("'c';");
        test_parse(&code);

        // Escaped character
        let code = String::from("'\\';");
        test_parse(&code);
    }

    #[test]
    fn test_parse_string_literal() {
        let code = String::from("let x = \"aD_67'le\";");

        test_parse(&code);
    }

    #[test]
    fn test_parse_boolean_literal() {
        let code = String::from("true;");
        test_parse(&code);

        let code = String::from("false;");
        test_parse(&code);
    }

    #[test]
    fn test_parse_operation() {
        let code = String::from("1 + 2;");
        test_parse(&code);

        let code = String::from("6 - 21;");
        test_parse(&code);

        let code = String::from("24 * 3;");
        test_parse(&code);

        let code = String::from("8 / 93;");
        test_parse(&code);
    }

    #[test]
    fn test_parse_comparison() {
        let code = String::from("1 == 2;");
        test_parse(&code);

        let code = String::from("6 != 21;");
        test_parse(&code);

        let code = String::from("24 > 3;");
        test_parse(&code);

        let code = String::from("8 < 93;");
        test_parse(&code);
    }
    
    #[test]
    fn test_parse_logical_operators() {
        let code = String::from("true && !false;");
        test_parse(&code);

        let code = String::from("y || !x;");
        test_parse(&code);
    }

    #[test]
    fn test_parse_group_expression() {
        let code = String::from("
            ((2 + 2) * 4) + 6;
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_prefix() {
        let code = String::from("-2;");
        test_parse(&code);

        let code = String::from("!4;");
        test_parse(&code);
    }

    #[test]
    fn test_parse_if_expression() {
        let code = String::from("
            if x == 1 {
                2 + 4;
            }
        ");
        test_parse(&code);

        let code = String::from("
            if x == 1 {
                2 + 4;
            } else {
                6 + 12;
            }
        ");
        test_parse(&code);

        let code = String::from("
            if x == 1 {
                2 + 4;
            } else if x > 2 {
                6 + 12;
            } else {
                32 - 1;
            }
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_while_expression() {
        let code = String::from("
            while x > 1 {
                2 + 4;
            }
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_function() {
        let code = String::from("
            fn myFunction() {
                2 + 4;
            }
        ");
        test_parse(&code);

        let code = String::from("
            fn myFunction(a: int, b: int) -> int {
                return a + b;
            }
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_call_expression() {
        let code = String::from("
            myFunction();
        ");
        test_parse(&code);

        let code = String::from("
            myFunction(a, b);
        ");
        test_parse(&code);

        let code = String::from("
            myFunction(otherFunc(), 2 + 2);
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_return_expression() {
        let code = String::from("
            return 2 + 2;
        ");
        test_parse(&code);

        let code = String::from("
            return myFunction();
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_assignment_expression() {
        let code = String::from("
            a = call();
        ");
        test_parse(&code);

        let code = String::from("
            b = a + b;
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_access_expression() {
        let code = String::from("
            x::y;
        ");
        test_parse(&code);

        let code = String::from("
            z::call();
        ");
        test_parse(&code);
    }

}