use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    String(String),
    Number(f64),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Object::Null => "",
            Object::String(value) => value,
            Object::Number(x) => &x.to_string(),
        };

        write!(f, "{repr}")
    }
}
