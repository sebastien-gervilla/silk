#[derive(Debug, Clone, Copy)]
pub enum Value {
    F64(f64),
    Boolean(bool),
}

impl Default for Value {
    fn default() -> Self {
        Value::F64(0.0)
    }
}

pub type Values = Vec<Value>;