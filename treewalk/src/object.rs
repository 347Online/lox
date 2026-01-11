use std::fmt::Display;

use crate::function::{Function, LoxFunction, NativeFn};

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    String(String),
    Number(f64),
    Boolean(bool),
    Fn(Function),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(x) => *x,

            _ => true,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Object::Nil => "nil",
            Object::String(value) => value,
            Object::Number(x) => &x.to_string(),
            Object::Boolean(x) => &x.to_string(),
            Object::Fn(fun) => &fun.to_string(),
        };

        write!(f, "{repr}")
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object::String(value.to_owned())
    }
}

impl From<f64> for Object {
    fn from(value: f64) -> Self {
        Object::Number(value)
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Object::Boolean(value)
    }
}

impl From<NativeFn> for Object {
    fn from(value: NativeFn) -> Self {
        Object::Fn(Function::Native(value))
    }
}

impl From<LoxFunction> for Object {
    fn from(value: LoxFunction) -> Self {
        Object::Fn(Function::Lox(value))
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Nil, Object::Nil) => true,
            (Object::Nil, _) => false,

            (Object::String(lhs), Object::String(rhs)) => lhs == rhs,
            (Object::Number(lhs), Object::Number(rhs)) => lhs == rhs,
            (Object::Boolean(lhs), Object::Boolean(rhs)) => lhs == rhs,

            _ => false,
        }
    }
}
