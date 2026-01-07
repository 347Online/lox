use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::error::RuntimeError;
use crate::object::Object;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    Literal(Object),
    Grouping(SubExpr<'src>),
    Unary(Token<'src>, SubExpr<'src>),
    Binary(Token<'src>, SubExpr<'src>, SubExpr<'src>),
}

impl<'src> Expr<'src> {
    pub fn literal<T>(value: T) -> Self
    where
        Object: From<T>,
    {
        Expr::Literal(Object::from(value))
    }

    pub fn nil() -> Self {
        Expr::Literal(Object::Nil)
    }

    pub fn evaluate(&self) -> Result<Object, RuntimeError<'src>> {
        let value = match self {
            Expr::Literal(value) => value.clone(),
            Expr::Grouping(expr) => expr.evaluate()?,

            Expr::Unary(token, rhs) => match token.kind {
                TokenType::Bang => (!rhs.evaluate()?.is_truthy()).into(),

                TokenType::Minus => {
                    if let Expr::Literal(Object::Number(value)) = &**rhs {
                        Object::Number(-value)
                    } else {
                        return Err(RuntimeError::num(token.clone()));
                    }
                }

                _ => unreachable!("no other unary expression"),
            },

            Expr::Binary(token, lhs, rhs) => {
                let (lhs, rhs) = (lhs.evaluate()?, rhs.evaluate()?);

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
        };

        Ok(value)
    }
}

impl<'src> Default for Expr<'src> {
    fn default() -> Self {
        Expr::nil()
    }
}

#[derive(Clone)]
pub struct SubExpr<'a>(Box<Expr<'a>>);

impl Debug for SubExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<Expr<'a>> for SubExpr<'a> {
    fn from(value: Expr<'a>) -> Self {
        SubExpr(Box::new(value))
    }
}

impl<'a> Deref for SubExpr<'a> {
    type Target = Expr<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for SubExpr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
