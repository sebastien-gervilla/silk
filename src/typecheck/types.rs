use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Void,
    Integer,
    Boolean,
    String,
    Function(Vec<Type>, Box<Type>)
}

pub type TypeEnvironment = HashMap<String, Type>;