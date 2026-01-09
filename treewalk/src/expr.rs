use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    Assign {
        name: Token<'src>,
        value: SubExpr<'src>,
    },
    Binary {
        op: Token<'src>,
        lhs: SubExpr<'src>,
        rhs: SubExpr<'src>,
    },
    Grouping {
        expr: SubExpr<'src>,
    },
    Logical {
        op: Token<'src>,
        lhs: SubExpr<'src>,
        rhs: SubExpr<'src>,
    },
    Literal {
        value: Object,
    },
    Unary {
        op: Token<'src>,
        rhs: SubExpr<'src>,
    },
    Variable {
        name: Token<'src>,
    },
}

impl<'src> Expr<'src> {
    pub fn literal<T>(value: T) -> Self
    where
        Object: From<T>,
    {
        let value = value.into();

        Expr::Literal { value }
    }

    pub fn nil() -> Self {
        Expr::Literal { value: Object::Nil }
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

impl<'a> AsRef<Expr<'a>> for SubExpr<'a> {
    fn as_ref(&self) -> &Expr<'a> {
        self.0.as_ref()
    }
}

impl<'a> AsMut<Expr<'a>> for SubExpr<'a> {
    fn as_mut(&mut self) -> &mut Expr<'a> {
        self.0.as_mut()
    }
}

impl<'a> Deref for SubExpr<'a> {
    type Target = Expr<'a>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a> DerefMut for SubExpr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
