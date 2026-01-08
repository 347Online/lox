use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::lox::{Lox, LoxState};
use crate::object::Object;
use crate::token::TokenType;

#[derive(Debug)]
pub struct Interpreter {
    state: Rc<RefCell<LoxState>>,
    globals: Environment,
}

impl<'src> Interpreter {
    pub fn new(state: Rc<RefCell<LoxState>>) -> Self {
        let globals = Environment::new();

        Interpreter { state, globals }
    }

    fn evaluate(expr: &Expr<'src>, env: &mut Environment) -> Result<Object, RuntimeError<'src>> {
        let value = match expr {
            Expr::Literal(value) => value.clone(),
            Expr::Grouping(expr) => Self::evaluate(expr.deref(), env)?,
            Expr::Unary(token, rhs) => match token.kind {
                TokenType::Bang => (!Self::evaluate(rhs.deref(), env)?.is_truthy()).into(),

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
                let (lhs, rhs) = (
                    Self::evaluate(lhs.as_ref(), env)?,
                    Self::evaluate(rhs.as_ref(), env)?,
                );

                macro_rules! binary {
                    ($op:tt, $kind:tt) => {{
                        if let (Object::Number(lhs), Object::Number(rhs)) = (lhs, rhs) {
                            Ok(Object::$kind(lhs $op rhs))
                        } else {
                            Err(RuntimeError::num_pair(token.clone()))
                        }
                    }};
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
            Expr::Variable(token) => env.get(token)?.clone(),
            Expr::Assign(name, expr) => {
                let value = Self::evaluate(expr, env)?;
                env.assign(name, &value)?;

                value
            }
        };

        Ok(value)
    }

    fn execute_block(
        statements: &[Stmt<'src>],
        env: &mut Environment,
    ) -> Result<(), RuntimeError<'src>> {
        for stmt in statements {
            Self::execute(stmt, env)?;
        }

        Ok(())
    }

    fn execute(stmt: &Stmt<'src>, env: &mut Environment) -> Result<(), RuntimeError<'src>> {
        match stmt {
            Stmt::Expr(expr) => {
                Self::evaluate(expr, env)?;
            }
            Stmt::Print(expr) => {
                let value = Self::evaluate(expr, env)?;
                println!("{value}");
            }
            Stmt::Var(token, initializer) => {
                let value = if let Some(initializer) = initializer {
                    Self::evaluate(initializer, env)?
                } else {
                    Object::Nil
                };

                env.define(token.lexeme, value);
            }
            Stmt::Block(statements) => {
                Self::execute_block(statements, &mut Environment::new_enclosed(env.clone()))?;
            }
        }

        Ok(())
    }

    fn try_interpret(
        statements: Vec<Stmt<'src>>,
        env: &mut Environment,
    ) -> Result<(), RuntimeError<'src>> {
        for stmt in statements {
            Self::execute(&stmt, env)?;
        }

        Ok(())
    }

    pub fn interpret(&mut self, statements: Vec<Stmt<'src>>) {
        let mut globals = self.globals.clone();

        match Self::try_interpret(statements, &mut globals) {
            Ok(_) => (),
            Err(err) => Lox::runtime_error(self.state.borrow_mut(), err),
        }

        self.globals = globals;
    }
}
