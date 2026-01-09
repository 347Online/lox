use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::expr::Expr;
use crate::lox::{Lox, LoxState};
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::TokenType;

#[derive(Debug)]
pub struct Interpreter {
    state: Rc<RefCell<LoxState>>,
    environment: Rc<RefCell<Environment>>,
}

impl<'src> Interpreter {
    pub fn new(state: Rc<RefCell<LoxState>>) -> Self {
        let environment = Environment::new();

        Interpreter { state, environment }
    }

    fn evaluate(&mut self, expr: &Expr<'src>) -> Result<Object, RuntimeError<'src>> {
        let value = match expr {
            Expr::Literal(value) => value.clone(),
            Expr::Grouping(expr) => self.evaluate(expr.deref())?,
            Expr::Unary(token, rhs) => match token.kind {
                TokenType::Bang => (!self.evaluate(rhs.deref())?.is_truthy()).into(),

                TokenType::Minus => {
                    if let Expr::Literal(Object::Number(value)) = **rhs {
                        Object::Number(-value)
                    } else {
                        return Err(RuntimeError::num(token.clone()));
                    }
                }

                _ => unreachable!("no other unary expression"),
            },
            Expr::Binary(token, lhs, rhs) => {
                let (lhs, rhs) = (self.evaluate(lhs.as_ref())?, self.evaluate(rhs.as_ref())?);

                macro_rules! binary {
                    ($op:tt, $kind:tt) => {
                        if let (Object::Number(lhs), Object::Number(rhs)) = (lhs, rhs) {
                            Ok(Object::$kind(lhs $op rhs))
                        } else {
                            Err(RuntimeError::num_pair(token.clone()))
                        }
                    };
                }

                match token.kind {
                    TokenType::Minus => binary!(-, Number)?,
                    TokenType::Slash => binary!(/, Number)?,
                    TokenType::Star => binary!(*, Number)?,

                    TokenType::Plus => match (lhs, rhs) {
                        (Object::Number(lhs), Object::Number(rhs)) => (lhs + rhs).into(),
                        (Object::String(lhs), Object::String(rhs)) => (lhs + &rhs).as_str().into(),

                        _ => {
                            return Err(RuntimeError::nums_or_strings(token.clone()));
                        }
                    },

                    TokenType::Greater => binary!(>, Boolean)?,
                    TokenType::GreaterEqual => binary!( >=, Boolean)?,
                    TokenType::Less => binary!(<, Boolean)?,
                    TokenType::LessEqual => binary!(<=, Boolean)?,

                    TokenType::BangEqual => (lhs != rhs).into(),
                    TokenType::EqualEqual => (lhs == rhs).into(),

                    _ => unreachable!(),
                }
            }
            Expr::Variable(token) => self.environment.borrow().get(token)?.clone(),
            Expr::Assign(name, expr) => {
                let value = self.evaluate(expr)?;
                self.environment.borrow_mut().assign(name, &value)?;

                value
            }
        };

        Ok(value)
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt<'src>],
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError<'src>> {
        let previous = self.environment.clone();

        let result = 'block: {
            self.environment = environment;

            for stmt in statements {
                match self.execute(stmt) {
                    Ok(_) => (),
                    x => break 'block x,
                }
            }

            Ok(())
        };

        self.environment = previous;

        result
    }

    fn execute(&mut self, stmt: &Stmt<'src>) -> Result<(), RuntimeError<'src>> {
        match stmt {
            Stmt::Expr(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{value}");
            }
            Stmt::Var(token, initializer) => {
                let value = if let Some(initializer) = initializer {
                    self.evaluate(initializer)?
                } else {
                    Object::Nil
                };

                self.environment.borrow_mut().define(token.lexeme, value);
            }
            Stmt::Block(statements) => {
                self.execute_block(
                    statements,
                    Environment::new_enclosed(self.environment.clone()),
                )?;
            }
        }

        Ok(())
    }

    pub fn interpret(&mut self, statements: Vec<Stmt<'src>>) {
        let result = 'block: {
            for stmt in &statements {
                match self.execute(stmt) {
                    Ok(_) => (),
                    x => break 'block x,
                }
            }

            Ok(())
        };

        match result {
            Ok(_) => (),
            Err(err) => Lox::runtime_error(self.state.borrow_mut(), err),
        }
    }
}
