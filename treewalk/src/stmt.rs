use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt<'src> {
    Block(Vec<Stmt<'src>>),
    Expr(Expr<'src>),
    Print(Expr<'src>),
    Var(Token<'src>, Option<Expr<'src>>),
}
