use super::value::Value;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum OperationCode {
    UNKNOW,
    CONSTANT,
    TRUE,
    FALSE,
    ADD,
    SUBSTRACT,
    MULTIPLY,
    DIVIDE,
    EQUALS,
    NOT_EQUALS,
    GREATER,
    LESS,
    NOT,
    NEGATE,
    SET_GLOBAL,
    GET_GLOBAL,
    SET_LOCAL,
    GET_LOCAL,
    JUMP,
    JUMP_IF_FALSE,
    LOOP,
    CALL,
    RETURN,
    POP,
}

impl OperationCode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => OperationCode::CONSTANT,
            2 => OperationCode::TRUE,
            3 => OperationCode::FALSE,
            4 => OperationCode::ADD,
            5 => OperationCode::SUBSTRACT,
            6 => OperationCode::MULTIPLY,
            7 => OperationCode::DIVIDE,
            8 => OperationCode::EQUALS,
            9 => OperationCode::NOT_EQUALS,
            10 => OperationCode::GREATER,
            11 => OperationCode::LESS,
            12 => OperationCode::NOT,
            13 => OperationCode::NEGATE,
            14 => OperationCode::SET_GLOBAL,
            15 => OperationCode::GET_GLOBAL,
            16 => OperationCode::SET_LOCAL,
            17 => OperationCode::GET_LOCAL,
            18 => OperationCode::JUMP,
            19 => OperationCode::JUMP_IF_FALSE,
            20 => OperationCode::LOOP,
            21 => OperationCode::CALL,
            22 => OperationCode::RETURN,
            23 => OperationCode::POP,
            unknown => {
                println!("Unknown instruction '{}'", unknown);
                OperationCode::UNKNOW
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub contants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            contants: vec![],
            lines: vec![],
        }
    }

    pub fn push_constant(&mut self, constant: Value) -> u8 {
        self.contants.push(constant);
        return (self.contants.len() - 1) as u8;
    }

    pub fn add_constant(&mut self, value: Value, line: usize) {
        self.add_operation(OperationCode::CONSTANT, line);
        let constant = self.push_constant(value);
        self.add_instruction(constant, line);
    }

    pub fn add_instruction(&mut self, instruction: u8, line: usize) {
        self.code.push(instruction as u8);
        self.lines.push(line);
    }

    pub fn add_operation(&mut self, operation: OperationCode, line: usize) {
        self.code.push(operation as u8);
        self.lines.push(line);
    }

    pub fn add_jump(&mut self, operation: OperationCode, line: usize) -> usize {
        self.add_operation(operation, line);
        self.add_instruction(u8::MAX, line);
        self.add_instruction(u8::MAX, line);
        return self.code.len() - 2
    }

    pub fn patch_jump(&mut self, offset: usize) {
        // -2 is for the 2 u8 placeholders
        println!("{}-{}", self.code.len(), offset);
        let jump = self.code.len() - offset - 2;

        if jump > u16::MAX as usize {
            panic!("Cannot jump over that much code");
        }

        self.code[offset] = ((jump >> 8) as u8) & u8::MAX;
        self.code[offset + 1] = jump as u8 & u8::MAX;
    }

    pub fn add_loop(&mut self, loop_start: usize, line: usize) {
        self.add_operation(OperationCode::LOOP, line);

        let offset = self.code.len() - loop_start + 2;
        if offset > u16::MAX as usize {
            panic!("Loop body is too large");
        }

        self.add_instruction((offset >> 8) as u8, line);
        self.add_instruction(offset as u8, line);
    }

}