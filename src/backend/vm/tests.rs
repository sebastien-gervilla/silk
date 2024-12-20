#[cfg(test)]
mod tests {
    use crate::backend::{
        bytecode::{
            Chunk,
            OperationCode
        }, 
        object::FunctionObject, 
        value::Value, 
        vm::VM, 
        compiler::Compiler,
    };

    use crate::frontend::{
        lexer::Lexer, 
        parser::{parse_file, Parser}, 
        typecheck::check_program,
    };

    // Compilation tests

    fn test_compilation(source: &str) {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
    
        let ast = parse_file(&mut parser);
        println!("Parsing completed.");
    
        check_program(&ast);
        println!("Typechecking completed.");

        let function = &mut FunctionObject {
            chunk: Chunk::new(),
            arity: 0,
            name: String::from("Global"),
        };
    
        let mut compiler = Compiler::new(function);
        let function = compiler.compile(&ast);
    
        let mut vm = VM::new(function);
        vm.run();
    }

    #[test]
    fn test_compile_literals() {
        println!("\n======== Testing literals ========\n");
        test_compilation("-11;");
        test_compilation("22;");
        test_compilation("\"string\";");
        test_compilation("true;");
        test_compilation("false;");
    }

    #[test]
    fn test_compile_operations() {
        println!("\n======== Testing addition ========\n");
        test_compilation("3 + 22;");

        println!("\n======== Testing substraction ========\n");
        test_compilation("2 - 3;");

        println!("\n======== Testing multiplication ========\n");
        test_compilation("6 * 6;");

        println!("\n======== Testing division ========\n");
        test_compilation("10 / 2;");
    }

    #[test]
    fn test_compile_logical_operators() {
        println!("\n======== Testing and ========\n");
        test_compilation("true && (2 < 3);");

        println!("\n======== Testing and ========\n");
        test_compilation("false && (4 < 34);");

        println!("\n======== Testing or ========\n");
        test_compilation("true || (2 < 3);");

        println!("\n======== Testing or ========\n");
        test_compilation("false || (4 > 34);");
    }

    #[test]
    fn test_compile_negation() {
        println!("\n======== Testing negation ========\n");
        test_compilation("-22;");
    }

    #[test]
    fn test_compile_multiple_lines() {
        println!("\n======== Testing multiple lines ========\n");
        test_compilation("
            2 + 2;
            2 / 2;
        ");
    }

    #[test]
    fn test_compile_equality() {
        println!("\n======== Testing equality ========\n");
        test_compilation("
            2 == 2;
            2 != 2;
            true == false;
            true != false;
        ");
    }

    #[test]
    fn test_compile_block() {
        println!("\n======== Testing block ========\n");
        test_compilation("
            {
                let x = 2;
                x = x + 2;
            }
        ");
    }

    #[test]
    fn test_compile_if_expression() {
        println!("\n======== Testing if expression ========\n");
        test_compilation("
            if true {
                false;
            };
        ");

        test_compilation("
            if false {
                true;
            } else {
                if 2 > 1 {
                    false;
                };
            };
        ");
    }

    #[test]
    fn test_compile_while_expression() {
        println!("\n======== Testing while expression ========\n");
        test_compilation("
            while false {
                true;
            };
        ");
        
        test_compilation("
            {
                let x = 0;
                while x < 10 {
                    x = x + 1;
                };
            }
        ");
    }

    #[test]
    fn test_compile_array_expression() {
        println!("\n======== Testing array expression ========\n");
        test_compilation("
            {
                let x = [0, 1, 2];
            }
        ");
    }

    #[test]
    fn test_compile_index_expression() {
        println!("\n======== Testing index expression ========\n");
        test_compilation("
            {
                let x = [0, 1];
                if x[1] == 1 {
                    true;
                } else {
                    false;
                }
            }
        ");
    }

}