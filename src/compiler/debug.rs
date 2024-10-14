use crate::compiler::bytecode::OperationCode;

use super::bytecode::Chunk;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("===== {} =====", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset)
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("> OFFSET {} - ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!(" | ");
    } else {
        print!(" {} ", chunk.lines[offset]);
    }

    let instruction = OperationCode::from_u8(chunk.code[offset]);

    match instruction {
        OperationCode::CONSTANT => return handle_constant_instruction(chunk, offset),
        OperationCode::TRUE => return handle_simple_instruction("TRUE", offset),
        OperationCode::FALSE => return handle_simple_instruction("FALSE", offset),
        OperationCode::ADD => return handle_simple_instruction("ADD", offset),
        OperationCode::SUBSTRACT => return handle_simple_instruction("SUBSTRACT", offset),
        OperationCode::MULTIPLY => return handle_simple_instruction("MULTIPLY", offset),
        OperationCode::DIVIDE => return handle_simple_instruction("DIVIDE", offset),
        OperationCode::EQUALS => return handle_simple_instruction("EQUALS", offset),
        OperationCode::NOT_EQUALS => return handle_simple_instruction("NOT_EQUALS", offset),
        OperationCode::GREATER => return handle_simple_instruction("GREATER", offset),
        OperationCode::LESS => return handle_simple_instruction("LESS", offset),
        OperationCode::NOT => return handle_simple_instruction("NOT", offset),
        OperationCode::NEGATE => return handle_simple_instruction("NEGATE", offset),
        OperationCode::RETURN => return handle_simple_instruction("RETURN", offset),
        OperationCode::UNKNOW => println!("UNKNOW {:?}", instruction),
    }

    return offset + 1
}

fn handle_simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    return offset + 1
}

fn handle_constant_instruction(chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code[offset + 1];
    
    println!(
        "CONSTANT (VALUE: {:?}, index: {}) ", 
        chunk.contants[constant_index as usize], 
        constant_index
    );

    return offset + 2
}