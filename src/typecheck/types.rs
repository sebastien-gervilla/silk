use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Type {
    Integer,
    Boolean
}

pub type TypeEnvironment = HashMap<String, Type>;