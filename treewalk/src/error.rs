use std::fmt::Display;

use crate::token::Token;

pub struct ParseError;

pub struct RuntimeError<'src> {
    token: Token<'src>,
    message: String,
}

impl<'src> RuntimeError<'src> {
    pub fn new(token: Token<'src>, message: impl Into<String>) -> Self {
        let message = message.into();

        RuntimeError { token, message }
    }

    pub fn num(token: Token<'src>) -> Self {
        RuntimeError::new(token, "Operand must be a number.")
    }

    pub fn num_pair(token: Token<'src>) -> Self {
        RuntimeError::new(token, "Operands must be numbers.")
    }

    pub fn nums_or_strings(token: Token<'src>) -> Self {
        RuntimeError::new(token, "Operands must be two numbers or two strings.")
    }
}

impl Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.token.line)
    }
}
