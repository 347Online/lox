use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt<'src> {
    Block {
        statements: Vec<Stmt<'src>>,
    },
    Expr {
        expr: Expr<'src>,
    },
    If {
        condition: Expr<'src>,
        then_branch: SubStmt<'src>,
        else_branch: Option<SubStmt<'src>>,
    },
    Print {
        value: Expr<'src>,
    },
    Var {
        name: Token<'src>,
        initializer: Option<Expr<'src>>,
    },
}

#[derive(Clone)]
pub struct SubStmt<'a>(Box<Stmt<'a>>);

impl Debug for SubStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<Stmt<'a>> for SubStmt<'a> {
    fn from(value: Stmt<'a>) -> Self {
        SubStmt(Box::new(value))
    }
}

impl<'a> AsRef<Stmt<'a>> for SubStmt<'a> {
    fn as_ref(&self) -> &Stmt<'a> {
        self.0.as_ref()
    }
}

impl<'a> AsMut<Stmt<'a>> for SubStmt<'a> {
    fn as_mut(&mut self) -> &mut Stmt<'a> {
        self.0.as_mut()
    }
}

impl<'a> Deref for SubStmt<'a> {
    type Target = Stmt<'a>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a> DerefMut for SubStmt<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
