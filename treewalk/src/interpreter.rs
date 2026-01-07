use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Expr;
use crate::lox::{Lox, LoxState};

#[derive(Debug)]
pub struct Interpreter {
    state: Rc<RefCell<LoxState>>,
}

impl Interpreter {
    pub fn new(state: Rc<RefCell<LoxState>>) -> Self {
        Interpreter { state }
    }

    pub fn interpret(&mut self, expr: Expr) {
        match expr.evaluate() {
            Ok(value) => println!("{value}"),
            Err(err) => Lox::runtime_error(self.state.borrow_mut(), err),
        }
    }
}
