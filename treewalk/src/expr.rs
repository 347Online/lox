use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: SubExpr,
    },
    Binary {
        op: Token,
        lhs: SubExpr,
        rhs: SubExpr,
    },
    Call {
        callee: SubExpr,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expr: SubExpr,
    },
    Logical {
        op: Token,
        lhs: SubExpr,
        rhs: SubExpr,
    },
    Literal {
        value: Object,
    },
    Unary {
        op: Token,
        rhs: SubExpr,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
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

impl Default for Expr {
    fn default() -> Self {
        Expr::nil()
    }
}

#[derive(Clone)]
pub struct SubExpr(Box<Expr>);

impl Debug for SubExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Expr> for SubExpr {
    fn from(value: Expr) -> Self {
        SubExpr(Box::new(value))
    }
}

impl AsRef<Expr> for SubExpr {
    fn as_ref(&self) -> &Expr {
        self.0.as_ref()
    }
}

impl AsMut<Expr> for SubExpr {
    fn as_mut(&mut self) -> &mut Expr {
        self.0.as_mut()
    }
}

impl Deref for SubExpr {
    type Target = Expr;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for SubExpr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
