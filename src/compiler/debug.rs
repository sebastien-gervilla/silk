use crate::compiler::bytecode::OperationCode;

use super::bytecode::Chunk;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("===== {} =====", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset)
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("> OFFSET {} - ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!(" | ");
    } else {
        print!(" {} ", chunk.lines[offset]);
    }

    let instruction = OperationCode::from_u8(chunk.code[offset]);

    match instruction {
        OperationCode::RETURN => return handle_return_instruction(offset),
        OperationCode::CONSTANT => return handle_constant_instruction(chunk, offset),
        OperationCode::UNKNOW => println!("UNKNOW {:?}", instruction),
    }

    return offset + 1
}

fn handle_return_instruction(offset: usize) -> usize {
    println!("RETURN");
    return offset + 1
}

fn handle_constant_instruction(chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code[offset + 1];
    
    println!(
        "CONSTANT (VALUE: {}, index: {}) ", 
        chunk.contants[constant_index as usize], 
        constant_index
    );

    return offset + 2
}