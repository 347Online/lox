use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::object::Object;
use crate::token::Token;

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
