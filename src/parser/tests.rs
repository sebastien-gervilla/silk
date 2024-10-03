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

    #[test]
    fn test_parse_boolean() {
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
            };
        ");
        test_parse(&code);

        let code = String::from("
            if x == 1 {
                2 + 4;
            } else {
                6 + 12;
            };
        ");
        test_parse(&code);

        let code = String::from("
            if x == 1 {
                2 + 4;
            } else if x > 2 {
                6 + 12;
            } else {
                32 - 1;
            };
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_while_expression() {
        let code = String::from("
            while x > 1 {
                2 + 4;
            };
        ");
        test_parse(&code);
    }

    #[test]
    fn test_parse_function() {
        let code = String::from("
            fn myFunction() {
                2 + 4;
            };
        ");
        test_parse(&code);

        let code = String::from("
            fn myFunction(a, b, c) {
                2 + 4;
            };
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

}