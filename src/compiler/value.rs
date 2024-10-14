use super::object::Object;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<'a> {
    F64(f64),
    Boolean(bool),
    Object(&'a Object),
}

impl<'a> Default for Value<'a> {
    fn default() -> Self {
        Value::F64(0.0)
    }
}

pub type Values<'a> = Vec<Value<'a>>;