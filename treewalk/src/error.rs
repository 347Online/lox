use std::fmt::Display;

use crate::object::Object;
use crate::token::Token;

pub struct ParseError;

pub enum Exception {
    Error { token: Token, message: String },
    Return(Object),
}

impl Exception {
    pub fn new(token: Token, message: impl Into<String>) -> Self {
        let message = message.into();

        Exception::Error { token, message }
    }

    pub fn num(token: Token) -> Self {
        Exception::new(token, "Operand must be a number.")
    }

    pub fn num_pair(token: Token) -> Self {
        Exception::new(token, "Operands must be numbers.")
    }

    pub fn nums_or_strings(token: Token) -> Self {
        Exception::new(token, "Operands must be two numbers or two strings.")
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Exception::Error { token, message } => {
                write!(f, "{}\n[line {}]", message, token.line)
            }
            Exception::Return(x) => write!(f, "return {x};"),
        }
    }
}
