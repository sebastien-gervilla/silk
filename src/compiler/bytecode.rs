use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub enum OperationCode {
    UNKNOW,
    CONSTANT,
    RETURN,
}

impl OperationCode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => OperationCode::CONSTANT,
            2 => OperationCode::RETURN,
            _ => OperationCode::UNKNOW,
        }
    }
}

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

    pub fn add_instruction(&mut self, instruction: u8, line: usize) {
        self.code.push(instruction);
        self.lines.push(line);
    }
}