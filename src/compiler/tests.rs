#[cfg(test)]
mod tests {
    use crate::compiler::{
        bytecode::{
            Chunk,
            OperationCode
        }, object::FunctionObject, value::Value, vm::VM, Compiler
    };

    #[test]
    fn test_bytecode() {
        
        let mut chunk = Chunk::new();

        // CONSTANT 1.5
        chunk.add_constant(Value::F64(1.5), 1);

        // CONSTANT 2.5
        chunk.add_constant(Value::F64(2.5), 1);

        // ADD => 4
        chunk.add_operation(OperationCode::ADD, 1);

        // CONSTANT 4
        chunk.add_constant(Value::F64(4.5), 1);

        // DIVIDE => 1
        chunk.add_operation(OperationCode::DIVIDE, 1);

        // NEGATE => -1
        chunk.add_operation(OperationCode::NEGATE, 1);

        // RETURN => -1
        chunk.add_operation(OperationCode::RETURN, 1);

        // disassemble_chunk(&chunk, "Testing chunks");
        let mut vm = VM::new(&mut chunk);
        vm.run();
    }

    // Compilation tests

    fn test_compilation(source: &str) {
        let function = &mut FunctionObject {
            chunk: Chunk::new(),
            arity: 0,
            name: String::from("Global"),
        };
    
        let mut compiler = Compiler::new(function);
        let mut chunk = compiler.compile(source);
    
        let mut vm = VM::new(&mut chunk);
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

}