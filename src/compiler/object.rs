#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(StringObject),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringObject {
    // object: Object,
    pub length: usize,
    pub characters: Vec<char>,
}