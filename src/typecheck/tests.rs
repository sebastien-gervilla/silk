#[cfg(test)]
mod tests {
    use crate::{
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

}