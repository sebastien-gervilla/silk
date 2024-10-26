use super::bytecode::Chunk;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(StringObject),
    Function(FunctionObject),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringObject {
    pub length: usize,
    pub value: String,
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