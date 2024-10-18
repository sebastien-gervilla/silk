use std::array;

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
    chunk: &'a mut Chunk,
    ip: usize, // TODO: For the moment we use array indexing, but we may use pointer dereferencing instead of performance
    stack: [Value; STACK_SIZE],
    stack_top: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: array::from_fn(|_| Value::F64(0.0)),
            stack_top: 0,
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack = array::from_fn(|_| Value::F64(0.0))
    }

    pub fn stack_push(&mut self, value: Value) {
        if self.stack_top >= STACK_SIZE {
            panic!("Tried to push value {:?} to stack, but went out of bounds.", value);
        }

        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    pub fn stack_pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top].clone()
    }

    pub fn stack_peek(&mut self, distance: usize) -> Value {
        return self.stack[self.stack_top - distance - 1].clone()
    }

    pub fn run(&mut self) -> InterpretationResult {
        
        println!("{:?}", self.chunk.code);

        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                for index in 0..self.stack_top {
                    println!("------ STACK: [ {:?} ]", self.stack[index]);
                }
                disassemble_instruction(self.chunk, self.ip);
            }

            let instruction = OperationCode::from_u8(self.read_byte());

            match instruction {
                OperationCode::RETURN => {
                    println!("RETURN {:?}", self.stack_pop());
                    return InterpretationResult::OK
                },
                OperationCode::TRUE => self.stack_push(Value::Boolean(true)),
                OperationCode::FALSE => self.stack_push(Value::Boolean(false)),
                OperationCode::ADD => self.run_binary_operation(|a, b| a + b),
                OperationCode::SUBSTRACT => self.run_binary_operation(|a, b| a - b),
                OperationCode::MULTIPLY => self.run_binary_operation(|a, b| a * b),
                OperationCode::DIVIDE => self.run_binary_operation(|a, b| a / b),
                OperationCode::EQUALS => self.run_equality_operation(|a, b| a == b),
                OperationCode::NOT_EQUALS => self.run_equality_operation(|a, b| a != b),
                OperationCode::GREATER => self.run_comparison_operation(|a, b| a > b),
                OperationCode::LESS => self.run_comparison_operation(|a, b| a < b),
                OperationCode::NOT => self.run_not_operation(),
                OperationCode::NEGATE => self.run_negate_operation(),
                OperationCode::CONSTANT => self.run_constant_operation(),
                OperationCode::GET_LOCAL => self.run_get_local_operation(),
                OperationCode::SET_LOCAL => self.run_set_local_operation(),
                OperationCode::JUMP => self.run_jump_operation(),
                OperationCode::JUMP_IF_FALSE => self.run_jump_if_false_operation(),
                OperationCode::POP => { self.stack_pop(); },
                OperationCode::UNKNOW => panic!("Unknow instruction"),
            };

            if self.ip >= self.chunk.code.len() {
                #[cfg(feature = "debug_trace_execution")] {
                    println!("");
                    for index in 0..self.stack_top {
                        println!(">> END STACK: [ {:?} ]", self.stack[index]);
                    }
                }

                return InterpretationResult::OK
            }
        }
    }

    fn run_binary_operation(&mut self, operation: fn(f64, f64) -> f64) {
        let b = self.stack_pop();
        let a = self.stack_pop();

        if let Value::F64(a) = a {
            if let Value::F64(b) = b {
                return self.stack_push(Value::F64(operation(a, b)));
            }

            panic!("Expected right to be f64, instead got {:?}", b);
        }

        panic!("Expected left to be f64, instead got {:?}", a);
    }

    fn run_equality_operation(&mut self, operation: fn(Value, Value) -> bool) {
        let b = self.stack_pop();
        let a = self.stack_pop();

        if !is_same_value_type(&a, &b) {
            panic!("Type mismatch between {:?} and {:?}", a, b);
        }

        self.stack_push(Value::Boolean(operation(a, b)));
    }

    fn run_comparison_operation(&mut self, operation: fn(f64, f64) -> bool) {
        let b = self.stack_pop();
        let a = self.stack_pop();

        if let Value::F64(a) = a {
            if let Value::F64(b) = b {
                return self.stack_push(Value::Boolean(operation(a, b)));
            }

            panic!("Expected right to be f64, instead got {:?}", b);
        }

        panic!("Expected left to be f64, instead got {:?}", a);
    }

    fn run_not_operation(&mut self) {
        let value = self.stack_pop();
        if let Value::Boolean(value) = value {
            return self.stack_push(Value::Boolean(!value));
        }

        panic!("Expected left to be boolean, instead got {:?}", value);
    }

    fn run_negate_operation(&mut self) {
        let value = self.stack_pop();
        if let Value::F64(value) = value {
            return self.stack_push(Value::F64(-value));
        }

        panic!("Expected left to be f64, instead got {:?}", value);
    }

    fn run_constant_operation(&mut self) {
        let constant = self.read_constant();
        println!("PUSHED {:?}", constant);
        self.stack_push(constant);
    }

    fn run_get_local_operation(&mut self) {
        let slot = self.read_byte();
        let value = self.stack[slot as usize].clone();
        self.stack_push(value);
    }

    fn run_set_local_operation(&mut self) {
        let slot = self.read_byte();
        self.stack[slot as usize] = self.stack_peek(0);
    }

    fn run_jump_operation(&mut self) {
        let offset = self.read_short();
        self.ip += offset as usize;
    }

    fn run_jump_if_false_operation(&mut self) {
        let offset = self.read_short();
        let condition_value = self.stack_peek(0);

        match condition_value {
            Value::Boolean(condition) => {
                if !condition {
                    return self.ip += offset as usize
                }
            },
            _ => panic!("Expected condition to be bool, instead got {:?}", condition_value)
        }
    }

    // Utils

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.chunk.contants[byte as usize].clone()
    }

    fn read_short(&mut self) -> u16 {
        let value = ((self.chunk.code[self.ip] as u16) << 8) | (self.chunk.code[self.ip + 1] as u16);
        self.ip += 2;
        return value
    }

}

fn is_same_value_type(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Boolean(_), Value::Boolean(_)) => true,
        (Value::F64(_), Value::F64(_)) => true,
        (Value::Object(_), Value::Object(_)) => true,
        _ => false,
    }
}