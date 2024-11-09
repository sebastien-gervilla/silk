use super::{bytecode::Chunk, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(StringObject),
    Array(ArrayObject),
    Function(FunctionObject),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringObject {
    pub length: usize,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayObject {
    pub elements: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct FunctionObject {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

impl PartialEq for FunctionObject {
    fn eq(&self, _: &Self) -> bool {
        return false
    }
}