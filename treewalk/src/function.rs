use std::fmt::{Debug, Display};

use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::object::Object;

#[derive(Clone)]
pub struct NativeFn {
    arity: usize,
    code: fn(&mut Interpreter, &[Object]) -> Object,
}

impl NativeFn {
    pub fn new(arity: usize, code: fn(&mut Interpreter, &[Object]) -> Object) -> Self {
        NativeFn { arity, code }
    }
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

macro_rules! native_fn {
    ($arity:expr, $fn:expr) => {
        $crate::object::Object::Fn($crate::function::Function::Native(
            $crate::function::NativeFn::new($arity, $fn),
        ))
    };
    ($fn:expr) => {
        native_fn!(0, $fn)
    };
}

pub(crate) use native_fn;

impl<'src> Function {
    pub fn native(arity: usize, code: fn(&mut Interpreter, &[Object]) -> Object) -> Self {
        Function::Native(NativeFn { arity, code })
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
            Function::Native(f) => Ok((f.code)(interpreter, args)),
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
