use std::fmt::Display;

use crate::token::Token;

pub struct ParseError;

enum RuntimeErrorType {
    Number,
    NumberPair,
    NumberPairOrStringPair,
}

impl std::fmt::Display for RuntimeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Self::Number => "Operand must be a number.",
            Self::NumberPair => "Operands must be numbers.",
            Self::NumberPairOrStringPair => "Operands must be two numbers or two strings.",
        };

        write!(f, "{repr}")
    }
}

pub struct RuntimeError<'src> {
    token: Token<'src>,
    kind: RuntimeErrorType,
}

impl<'src> RuntimeError<'src> {
    fn new(token: Token<'src>, kind: RuntimeErrorType) -> Self {
        RuntimeError { token, kind }
    }

    pub fn num(token: Token<'src>) -> Self {
        RuntimeError::new(token, RuntimeErrorType::Number)
    }

    pub fn num_pair(token: Token<'src>) -> Self {
        RuntimeError::new(token, RuntimeErrorType::NumberPair)
    }

    pub fn nums_or_strings(token: Token<'src>) -> Self {
        RuntimeError::new(token, RuntimeErrorType::NumberPairOrStringPair)
    }
}

impl Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n[line {}]", self.kind, self.token.line)
    }
}
