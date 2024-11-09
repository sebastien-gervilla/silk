use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    None, // This happens when using "return"
    Void,
    Integer,
    Boolean,
    String,
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
}

pub type TypeEnvironment = HashMap<String, Type>;