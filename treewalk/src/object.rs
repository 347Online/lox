use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Object::Nil => "",
            Object::String(value) => value,
            Object::Number(x) => &x.to_string(),
            Object::Boolean(x) => &x.to_string(),
        };

        write!(f, "{repr}")
    }
}
