#[cfg(test)]
mod tests {
    use crate::compiler::{
        bytecode::{
            Chunk,
            OperationCode
        }, debug::disassemble_chunk
    };

    fn test_compilation() {
        let mut chunk = Chunk::new();
        chunk.add_instruction(OperationCode::RETURN as u8, 1);
        chunk.add_instruction(OperationCode::CONSTANT as u8, 1);

        chunk.contants.push(1.2);
        chunk.add_instruction((chunk.contants.len() - 1) as u8, 1);
        disassemble_chunk(&chunk, "Testing chunks");
    }

    #[test]
    fn test_bytecode_return() {
        test_compilation();
    }
}