use super::bytecode::{
    OperationCode,
    Chunk,
};

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
        OperationCode::CONSTANT => return handle_constant_instruction("CONSTANT", chunk, offset),
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
        OperationCode::SET_GLOBAL => return handle_constant_instruction("SET_GLOBAL", chunk, offset),
        OperationCode::GET_GLOBAL => return handle_constant_instruction("GET_GLOBAL", chunk, offset),
        OperationCode::SET_LOCAL => return handle_byte_instruction("SET_LOCAL", chunk, offset),
        OperationCode::GET_LOCAL => return handle_byte_instruction("GET_LOCAL", chunk, offset),
        OperationCode::JUMP => return handle_jump_instruction("JUMP", chunk, offset, 1),
        OperationCode::JUMP_IF_FALSE => return handle_jump_instruction("JUMP_IF_FALSE", chunk, offset, 1),
        OperationCode::LOOP => return handle_jump_instruction("LOOP", chunk, offset, -1),
        OperationCode::CALL => return handle_byte_instruction("CALL", chunk, offset),
        OperationCode::BUILD_ARRAY => return handle_byte_instruction("BUILD_ARRAY", chunk, offset),
        OperationCode::INDEX_ARRAY => return handle_byte_instruction("INDEX_ARRAY", chunk, offset),
        OperationCode::RETURN => return handle_simple_instruction("RETURN", offset),
        OperationCode::POP => return handle_simple_instruction("POP", offset),
        OperationCode::UNKNOW => println!("UNKNOW {:?}", instruction),
    }

    return offset + 1
}

fn handle_simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    return offset + 1
}

fn handle_byte_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let slot = chunk.code[offset + 1];
    println!("{} {}", name, slot);
    return offset + 2
}

fn handle_constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code[offset + 1];
    
    println!(
        "{name} (VALUE: {:?}, index: {}) ", 
        chunk.contants[constant_index as usize], 
        constant_index
    );

    return offset + 2
}

fn handle_jump_instruction(name: &str, chunk: &Chunk, offset: usize, sign: isize) -> usize {
    println!("jump {} {:?}", offset, chunk.code[offset + 1]);
    let mut jump = (chunk.code[offset + 1] as u16) << 8;

    jump |= chunk.code[offset + 2] as u16;

    let target = offset as isize + 3 + sign * jump as isize;
    println!("- {} {} -> {}", name, offset, target);
    return offset + 3
}