use super::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    F64(f64),
    Boolean(bool),
    Object(Object),
}

impl Default for Value {
    fn default() -> Self {
        Value::F64(0.0)
    }
}

pub type Values = Vec<Value>;