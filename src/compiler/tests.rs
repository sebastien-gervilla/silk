#[cfg(test)]
mod tests {
    use crate::compiler::{
        bytecode::{
            Chunk,
            OperationCode
        },
        vm::VM
    };

    fn test_compilation() {
        
        let mut chunk = Chunk::new();

        // CONSTANT 1.5
        chunk.add_constant(1.5, 1);

        // CONSTANT 2.5
        chunk.add_constant(2.5, 1);

        // ADD => 4
        chunk.add_operation(OperationCode::ADD, 1);

        // CONSTANT 4
        chunk.add_constant(4.0, 1);

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

    #[test]
    fn test_bytecode() {
        test_compilation();
    }
}