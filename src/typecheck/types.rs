use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Integer,
    Boolean,
    Function(Vec<Type>, Box<Type>)
}

pub type TypeEnvironment = HashMap<String, Type>;