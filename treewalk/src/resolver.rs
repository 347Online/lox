use std::collections::HashMap;

use crate::expr::{Expr, ExprData};
use crate::interpreter::Interpreter;
use crate::lox::Lox;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: vec![],
        }
    }

    pub fn finish(self) -> Interpreter {
        self.interpreter
    }

    pub fn resolve_statements(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        };

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_owned(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_owned(), true);
        }
    }

    fn resolve_local_expr(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i)
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match &expr.data {
            ExprData::Assign { name, value } => {
                self.resolve_expr(value);
                self.resolve_local_expr(expr, name);
            }
            ExprData::Binary { lhs, rhs, .. } | ExprData::Logical { lhs, rhs, .. } => {
                self.resolve_expr(lhs);
                self.resolve_expr(rhs);
            }
            ExprData::Call {
                callee, arguments, ..
            } => {
                self.resolve_expr(callee);
                for argument in arguments {
                    self.resolve_expr(argument);
                }
            }
            ExprData::Grouping { expr } => self.resolve_expr(expr),
            ExprData::Literal { .. } => (),
            ExprData::Unary { rhs, .. } => self.resolve_expr(rhs),
            ExprData::Variable { name } => {
                if let Some(scope) = self.scopes.last()
                    && let Some(false) = scope.get(&name.lexeme)
                {
                    Lox::error(
                        self.interpreter.state.borrow_mut(),
                        name.line,
                        "Can't read local variable in its own initializer.",
                    );
                }

                self.resolve_local_expr(expr, name);
            }
        }
    }

    fn resolve_function(&mut self, parameters: &[Token], body: &[Stmt]) {
        self.begin_scope();
        for param in parameters {
            self.declare(param);
            self.define(param);
        }
        self.resolve_statements(body);
        self.end_scope();
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_statements(statements);
                self.end_scope();
            }
            Stmt::Expr { expr } => self.resolve_expr(expr),
            Stmt::Function {
                name,
                parameters,
                body,
            } => {
                self.declare(name);
                self.define(name);
                self.resolve_function(parameters, body);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch);
                }
            }
            Stmt::Print { expr } => self.resolve_expr(expr),
            Stmt::Return { expr, .. } => {
                if let Some(expr) = expr {
                    self.resolve_expr(expr);
                }
            }
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(initializer) = initializer {
                    self.resolve_expr(initializer);
                }
                self.define(name);
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition);
                self.resolve_stmt(body);
            }
        }
    }
}
