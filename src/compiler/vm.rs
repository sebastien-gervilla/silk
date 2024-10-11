use super::{bytecode::{Chunk, OperationCode}, debug::disassemble_instruction, value::Value, Compiler};

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
            stack: [Value::default(); STACK_SIZE],
            stack_top: 0,
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack = [Value::default(); STACK_SIZE];
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
        self.stack[self.stack_top]
    }

    pub fn interpret(&mut self, source: &str) -> InterpretationResult {
        let mut compiler = Compiler::new(&mut self.chunk);
        compiler.compile(source);

        self.run()
    }

    pub fn run(&mut self) -> InterpretationResult {
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
                OperationCode::ADD => self.run_binary_operation(|a, b| a + b),
                OperationCode::SUBSTRACT => self.run_binary_operation(|a, b| a - b),
                OperationCode::MULTIPLY => self.run_binary_operation(|a, b| a * b),
                OperationCode::DIVIDE => self.run_binary_operation(|a, b| a / b),
                OperationCode::NEGATE => self.run_negate_operation(),
                OperationCode::CONSTANT => self.run_constant_operation(),
                OperationCode::UNKNOW => panic!("Unknow instruction")
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
                self.stack_push(Value::F64(operation(a, b)));
            }

            panic!("Expected right to be f64, instead got {:?}", b);
        }

        panic!("Expected left to be f64, instead got {:?}", a);
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
        self.stack_push(constant);
        println!("PUSHED {:?}", constant);
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.chunk.contants[byte as usize]
    }

}