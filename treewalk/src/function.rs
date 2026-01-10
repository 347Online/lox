use std::fmt::{Debug, Display};

use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::object::Object;

#[derive(Clone)]
pub struct NativeFn {
    arity: usize,
    fun: fn(&mut Interpreter, &[Object]) -> Object,
}

impl Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native code")
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Native(NativeFn),
}

impl<'src> Function {
    pub fn native(arity: usize, fun: fn(&mut Interpreter, &[Object]) -> Object) -> Self {
        Function::Native(NativeFn { arity, fun })
    }

    pub fn arity(&self) -> usize {
        match self {
            Function::Native(f) => f.arity,
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &[Object],
    ) -> Result<Object, RuntimeError<'src>> {
        match self {
            Function::Native(f) => Ok((f.fun)(interpreter, args)),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Function::Native(_) => "<native fn>",
        };

        write!(f, "{}", repr)
    }
}
