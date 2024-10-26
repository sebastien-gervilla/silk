use std::{
    array, 
    collections::HashMap
};

use super::{
    bytecode::OperationCode, 
    debug::disassemble_instruction, 
    object::{FunctionObject, Object}, 
    value::Value
};

const FRAMES_SIZE: usize = 64;
const STACK_SIZE: usize = 64 * 128;

type Globals = HashMap<String, Value>;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum InterpretationResult {
    OK,
    COMPILE_ERROR,
    RUNTIME_ERROR
}

pub struct CallFrame {
    pub function: FunctionObject,
    pub ip: usize, // TODO: For the moment we use array indexing, but we may use pointer dereferencing instead of performance
    pub slots: Vec<Value>,
}

pub struct VM {
    frames: [Option<CallFrame>; FRAMES_SIZE],
    frames_count: usize,

    stack: Vec<Value>,

    globals: Globals,
}

impl VM {
    pub fn new(function: &mut FunctionObject) -> Self {
        let mut frames: [Option<CallFrame>; FRAMES_SIZE] = array::from_fn(|_| None);
        frames[0] = Some(CallFrame {
            function: function.clone(),
            ip: 0,
            slots: vec![]
        });

        Self {
            frames,
            frames_count: 1,
            stack: Vec::with_capacity(STACK_SIZE),
            globals: Globals::new(),
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack = Vec::with_capacity(STACK_SIZE)
    }

    pub fn stack_push(&mut self, value: Value) {
        if self.stack.len() >= STACK_SIZE {
            panic!("Tried to push value {:?} to stack, but went out of bounds.", value);
        }

        self.stack.push(value);
    }

    pub fn stack_pop(&mut self) -> Value {
        self.stack.pop().expect("Tried to pop non existing value.")
    }

    pub fn stack_peek(&mut self, distance: usize) -> Value {
        return self.stack[self.stack.len() - distance - 1].clone()
    }

    pub fn run(&mut self) -> InterpretationResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                for index in 0..self.stack.len() {
                    println!("------ STACK: [ {:?} ]", self.stack[index]);
                }
                disassemble_instruction(self.chunk, self.ip);
            }

            let instruction = OperationCode::from_u8(self.read_byte());

            match instruction {
                OperationCode::RETURN => {
                    if self.run_return_operation() {
                        return InterpretationResult::OK
                    }
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
                OperationCode::LOOP => self.run_loop(),
                OperationCode::POP => { self.stack_pop(); },
                OperationCode::UNKNOW => panic!("Unknow instruction"),
            };

            if self.ip >= self.chunk.code.len() {
                #[cfg(feature = "debug_trace_execution")] {
                    println!("");
                    for index in 0..self.stack.len() {
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
        let frame = self.get_current_frame();
        let value = frame.slots[slot as usize].clone();
        self.stack_push(value);
    }

    fn run_set_local_operation(&mut self) {
        let slot = self.read_byte();
        let peek_value = self.stack_peek(0);
        let frame = self.get_current_frame();
        if frame.slots.len() <= slot as usize {
            frame.slots.push(peek_value);
        } else {
            frame.slots[slot as usize] = peek_value;
        }
    }

    fn run_jump_operation(&mut self) {
        let offset = self.read_short();
        let frame = self.get_current_frame();
        frame.ip += offset as usize;
    }

    fn run_jump_if_false_operation(&mut self) {
        let offset = self.read_short();
        let condition_value = self.stack_peek(0);

        match condition_value {
            Value::Boolean(condition) => {
                if !condition {
                    let frame = self.get_current_frame();
                    return frame.ip += offset as usize
                }
            },
            _ => panic!("Expected condition to be bool, instead got {:?}", condition_value)
        }
    }

    fn run_loop(&mut self) {
        let offset = self.read_short();
        let frame = self.get_current_frame();
        frame.ip -= offset as usize;
    }

    fn run_return_operation(&mut self) -> bool {
        let value = self.stack_pop();
        self.frames_count -= 1;
        if self.frames_count <= 0 {
            self.stack_pop();
            return true
        }

        self.stack_push(value);
        return false
    }


    // Utils

    fn get_current_frame(&mut self) -> &mut CallFrame {
        match &mut self.frames[self.frames_count - 1] {
            Some(frame) => frame,
            None => panic!("Couldn't find any frame"),
        }
    }

    fn read_byte(&mut self) -> u8 {
        let frame = self.get_current_frame();
        frame.ip += 1;
        frame.function.chunk.code[frame.ip - 1]
    }

    fn read_short(&mut self) -> u16 {
        let frame = self.get_current_frame();
        let value = ((frame.function.chunk.code[frame.ip] as u16) << 8) 
            | (frame.function.chunk.code[frame.ip + 1] as u16);
        frame.ip += 2;
        return value
    }

    fn read_constant(&mut self) -> Value {
        let frame = self.get_current_frame();
        frame.ip += 1;
        let byte = frame.function.chunk.code[frame.ip - 1];
        frame.function.chunk.contants[byte as usize].clone()
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