#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(StringObject),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringObject {
    pub length: usize,
    pub value: String,
}