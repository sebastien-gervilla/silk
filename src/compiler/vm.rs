use super::{bytecode::{Chunk, OperationCode}, debug::disassemble_instruction, value::Value};

const STACK_SIZE: usize = 256;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum InterpretationResult {
    OK,
    COMPILE_ERROR,
    RUNTIME_ERROR
}

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize, // TODO: For the moment we use array indexing, but we may use pointer dereferencing instead of performance
    stack: [Value; STACK_SIZE],
    stack_top: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: [Value::default(); STACK_SIZE],
            stack_top: 0,
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack = [Value::default(); STACK_SIZE];
    }

    pub fn stack_push(&mut self, value: Value) {
        if self.stack_top >= STACK_SIZE {
            panic!("Tried to push value {} to stack, but went out of bounds.", value);
        }

        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    pub fn stack_pop(&mut self) -> f64 {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    pub fn interpret(&mut self) -> InterpretationResult {
        self.run()
    }

    fn run(&mut self) -> InterpretationResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                for index in 0..self.stack_top {
                    println!("------ STACK: [ {} ]", self.stack[index]);
                }
                disassemble_instruction(self.chunk, self.ip);
            }

            let instruction = OperationCode::from_u8(self.read_byte());

            match instruction {
                OperationCode::RETURN => {
                    println!("RETURN {}", self.stack_pop());
                    return InterpretationResult::OK
                },
                OperationCode::ADD => self.run_binary_operation(|a, b| a + b),
                OperationCode::SUBSTRACT => self.run_binary_operation(|a, b| a - b),
                OperationCode::MULTIPLY => self.run_binary_operation(|a, b| a * b),
                OperationCode::DIVIDE => self.run_binary_operation(|a, b| a / b),
                OperationCode::NEGATE => {
                    let value = self.stack_pop();
                    self.stack_push(-value);
                },
                OperationCode::CONSTANT => {
                    let constant = self.read_constant();
                    self.stack_push(constant);
                    println!("PUSHED {}", constant);
                },
                OperationCode::UNKNOW => panic!("Unknow instruction")
            };
        }
    }

    fn run_binary_operation(&mut self, operation: fn(f64, f64) -> f64) {
        let b = self.stack_pop();
        let a = self.stack_pop();
        self.stack_push(operation(a, b));
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self) -> f64 {
        self.chunk.contants[self.read_byte() as usize]
    }

}