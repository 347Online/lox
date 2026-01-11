use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use uuid::Uuid;

use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum ExprData {
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

#[derive(Debug, Clone)]
pub struct Expr {
    pub data: ExprData,
    id: Uuid,
}

impl Expr {
    pub(crate) fn new(data: ExprData) -> Self {
        Expr {
            data,
            id: Uuid::new_v4(),
        }
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::new(ExprData::Assign {
            name,
            value: value.into(),
        })
    }

    pub fn binary(op: Token, lhs: Expr, rhs: Expr) -> Self {
        Expr::new(ExprData::Binary {
            op,
            lhs: lhs.into(),
            rhs: rhs.into(),
        })
    }

    pub fn call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Expr::new(ExprData::Call {
            callee: callee.into(),
            paren,
            arguments,
        })
    }

    pub fn grouping(expr: Expr) -> Self {
        Expr::new(ExprData::Grouping { expr: expr.into() })
    }

    pub fn logical(op: Token, lhs: Expr, rhs: Expr) -> Self {
        Expr::new(ExprData::Logical {
            op,
            lhs: lhs.into(),
            rhs: rhs.into(),
        })
    }

    pub fn literal<T>(value: T) -> Self
    where
        Object: From<T>,
    {
        Expr::new(ExprData::Literal {
            value: value.into(),
        })
    }

    pub fn unary(op: Token, rhs: Expr) -> Self {
        Expr::new(ExprData::Unary {
            op,
            rhs: rhs.into(),
        })
    }

    pub fn variable(name: Token) -> Self {
        Expr::new(ExprData::Variable { name })
    }

    pub fn nil() -> Self {
        Expr::new(ExprData::Literal { value: Object::Nil })
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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
