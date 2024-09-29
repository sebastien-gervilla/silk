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
        chunk.add_instruction(OperationCode::CONSTANT as u8, 1);
        chunk.contants.push(1.5);
        chunk.add_instruction((chunk.contants.len() - 1) as u8, 1);

        // CONSTANT 2.5
        chunk.add_instruction(OperationCode::CONSTANT as u8, 1);
        chunk.contants.push(2.5);
        chunk.add_instruction((chunk.contants.len() - 1) as u8, 1);

        // ADD => 4
        chunk.add_instruction(OperationCode::ADD as u8, 1);

        // CONSTANT 4
        chunk.add_instruction(OperationCode::CONSTANT as u8, 1);
        chunk.contants.push(4.0);
        chunk.add_instruction((chunk.contants.len() - 1) as u8, 1);

        // DIVIDE => 1
        chunk.add_instruction(OperationCode::DIVIDE as u8, 1);

        // NEGATE => -1
        chunk.add_instruction(OperationCode::NEGATE as u8, 1);

        // RETURN => -1
        chunk.add_instruction(OperationCode::RETURN as u8, 1);

        // disassemble_chunk(&chunk, "Testing chunks");
        let mut vm = VM::new(&chunk);
        vm.interpret();
    }

    #[test]
    fn test_bytecode() {
        test_compilation();
    }
}