use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::error::RuntimeError;
use crate::lox::{Lox, LoxState};

#[derive(Debug)]
pub struct Interpreter {
    state: Rc<RefCell<LoxState>>,
}

impl<'src> Interpreter {
    pub fn new(state: Rc<RefCell<LoxState>>) -> Self {
        Interpreter { state }
    }

    fn try_interpret(&mut self, statements: Vec<Stmt<'src>>) -> Result<(), RuntimeError<'src>> {
        for stmt in statements {
            stmt.execute()?;
        }

        Ok(())
    }

    pub fn interpret(&mut self, statements: Vec<Stmt<'src>>) {
        match self.try_interpret(statements) {
            Ok(_) => (),
            Err(err) => Lox::runtime_error(self.state.borrow_mut(), err),
        }
    }
}
