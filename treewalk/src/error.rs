use std::fmt::Display;

use crate::token::Token;

pub struct ParseError;

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: impl Into<String>) -> Self {
        let message = message.into();

        RuntimeError { token, message }
    }

    pub fn num(token: Token) -> Self {
        RuntimeError::new(token, "Operand must be a number.")
    }

    pub fn num_pair(token: Token) -> Self {
        RuntimeError::new(token, "Operands must be numbers.")
    }

    pub fn nums_or_strings(token: Token) -> Self {
        RuntimeError::new(token, "Operands must be two numbers or two strings.")
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.token.line)
    }
}
