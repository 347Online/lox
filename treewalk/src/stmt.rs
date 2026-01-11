use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expr {
        expr: Expr,
    },
    Function {
        name: Token,
        parameters: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: SubStmt,
        else_branch: Option<SubStmt>,
    },
    Print {
        value: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: SubStmt,
    },
}

#[derive(Clone)]
pub struct SubStmt(Box<Stmt>);

impl Debug for SubStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Stmt> for SubStmt {
    fn from(value: Stmt) -> Self {
        SubStmt(Box::new(value))
    }
}

impl AsRef<Stmt> for SubStmt {
    fn as_ref(&self) -> &Stmt {
        self.0.as_ref()
    }
}

impl AsMut<Stmt> for SubStmt {
    fn as_mut(&mut self) -> &mut Stmt {
        self.0.as_mut()
    }
}

impl Deref for SubStmt {
    type Target = Stmt;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for SubStmt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
