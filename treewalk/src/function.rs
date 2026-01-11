use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::Exception;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::Token;

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
        f.debug_struct("NativeFn")
            .field("arity", &self.arity)
            .field_with("code", |f| write!(f, "<$NATIVE>"))
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    name: Token,
    parameters: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(
        name: Token,
        parameters: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        LoxFunction {
            name,
            parameters,
            body,
            closure,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Native(NativeFn),
    Lox(LoxFunction),
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

impl Function {
    pub fn native(arity: usize, code: fn(&mut Interpreter, &[Object]) -> Object) -> Self {
        Function::Native(NativeFn { arity, code })
    }

    pub fn arity(&self) -> usize {
        match self {
            Function::Native(f) => f.arity,
            Function::Lox(declaration) => declaration.parameters.len(),
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, Exception> {
        let value = match self {
            Function::Native(f) => (f.code)(interpreter, arguments),

            Function::Lox(declaration) => {
                let environment = Environment::new_enclosed(declaration.closure.clone());
                for (i, param) in declaration.parameters.iter().enumerate() {
                    environment
                        .borrow_mut()
                        .define(&param.lexeme, &arguments[i]);
                }

                let result = interpreter.execute_block(&declaration.body, environment);

                if let Err(Exception::Return(value)) = result {
                    return Ok(value);
                } else {
                    result?; // Propagate actual errors
                }

                Object::Nil
            }
        };

        Ok(value)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Function::Native(_) => "<native fn>",
            Function::Lox(declaration) => &format!("<fn {}>", declaration.name.lexeme),
        };

        write!(f, "{}", repr)
    }
}
