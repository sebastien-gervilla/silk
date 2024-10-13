use super::value::Value;

#[derive(Debug, Clone, Copy)]
pub enum OperationCode {
    UNKNOW,
    CONSTANT,
    ADD,
    SUBSTRACT,
    MULTIPLY,
    DIVIDE,
    NEGATE,
    RETURN,
}

impl OperationCode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => OperationCode::CONSTANT,
            2 => OperationCode::ADD,
            3 => OperationCode::SUBSTRACT,
            4 => OperationCode::MULTIPLY,
            5 => OperationCode::DIVIDE,
            6 => OperationCode::NEGATE,
            7 => OperationCode::RETURN,
            unknown => {
                println!("Unknown instruction '{}'", unknown);
                OperationCode::UNKNOW
            },
        }
    }
}

pub struct Chunk<'a> {
    pub code: Vec<u8>,
    pub contants: Vec<Value<'a>>,
    pub lines: Vec<usize>,
}

impl<'a> Chunk<'a> {
    pub fn new() -> Self {
        Self {
            code: vec![],
            contants: vec![],
            lines: vec![],
        }
    }

    pub fn add_constant(&mut self, constant: Value<'a>, line: usize) {
        self.add_operation(OperationCode::CONSTANT, line);
        self.contants.push(constant);
        self.add_instruction((self.contants.len() - 1) as u8, line);
    }

    pub fn add_instruction(&mut self, instruction: u8, line: usize) {
        self.code.push(instruction as u8);
        self.lines.push(line);
    }

    pub fn add_operation(&mut self, operation: OperationCode, line: usize) {
        self.code.push(operation as u8);
        self.lines.push(line);
    }
}