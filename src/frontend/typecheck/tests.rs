#[cfg(test)]
mod tests {
    use crate::frontend::{
        lexer::Lexer, 
        parser::{
            parse_file, 
            Parser
        }, typecheck::check_program
    };

    fn test_typecheck(code: &str) {
        let mut lexer = Lexer::new(code);
        let mut parser = Parser::new(&mut lexer);

        let ast_file = parse_file(&mut parser);

        if parser.errors.len() > 0 {
            for error in parser.errors {
                println!("{error}");
            }

            panic!("Found parsing errors.");
        }

        check_program(&ast_file);
    }

    #[test]
    fn test_typecheck_literals() {
        let code = "123;";
        test_typecheck(code);

        let code = "true;";
        test_typecheck(code);

        let code = "\"string\";";
        test_typecheck(code);
    }

    #[test]
    #[should_panic]
    fn test_typecheck_operation_different_types() {
        // Same type - Wrong Type
        let code = "true + true;";
        test_typecheck(code);
    }

    #[test]
    #[should_panic]
    fn test_typecheck_operation_wrong_types() {
        // Different types - Wrong type
        let code = "true - 1;";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_operations() {
        // Same types - Right type
        let code = "1 * 8;";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_complex_operation() {
        // Same types - Right type
        let code = "1 * 8 + 12;";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_logical_operations() {
        let code = "let x: bool = true && false;";
        test_typecheck(code);

        let code = "let y: bool = true || (2 < 3);";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_let_statement() {
        let code = "let x = 8;";
        test_typecheck(code);

        let code = "let x = (2 + 2);";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_function() {
        let code = "
            fn my_function() -> int {
                let x = 10;
                return if true {
                    x;
                } else {
                    return 2;
                };
            };
        ";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_call_expression() {
        let code = "
            fn my_function() -> int {
                let x = 10;
            };

            my_function();
        ";
        test_typecheck(code);

        let code = "
            fn my_function() -> int {
                let x = 10;
            };

            let x = my_function() + 4;
        ";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_identifier() {
        let code = "
            let x = 3;
            let y: int = x;
        ";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_if_expression() {
        let code = "
            let x: int = if 1 > 2 {
                15;
            } else {
                20;
            };
            
        ";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_prefix_expression() {
        let code = "
            let x: int = -5;  
        ";
        test_typecheck(code);
        
        let code = "
            if !true {
                -5;
            };
        ";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_while_expression() {
        let code = "
            while true {
                let x = 5;
                12;
            };
        ";
        test_typecheck(code);
    }

    #[test]
    #[should_panic]
    fn test_typecheck_wrong_nested_blocks() {
        let code = "
            if true {
                if true {
                    let x = 5;
                };

                let y: int = x;
            };
        ";
        test_typecheck(code);
    }

    #[test]
    fn test_typecheck_array_expression() {
        let code = "
            let x: [int] = [1, 2, 3];
        ";
        test_typecheck(code);
    }

    #[test]
    #[should_panic]
    fn test_typecheck_wrong_array_expression() {
        let code = "
            let x: [bool] = [true, 2, 3];
        ";
        test_typecheck(code);
    }

}