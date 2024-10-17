#[cfg(test)]
mod tests {
    use crate::compiler::{
        bytecode::{
            Chunk,
            OperationCode
        }, value::Value, vm::VM, Compiler
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
        let mut chunk = &mut Chunk::new();
        let mut compiler = Compiler::new(chunk);
        chunk = compiler.compile(source);
    
        let mut vm = VM::new(&mut chunk);
        vm.run();
    }

    #[test]
    fn test_compile_literals() {
        println!("\n======== Testing literals ========\n");
        test_compilation("22;");
        test_compilation("\"string\";");
        test_compilation("true;");
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

}