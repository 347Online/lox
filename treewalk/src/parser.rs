use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{Expr, Stmt};
use crate::error::ParseError;
use crate::lox::{Lox, LoxState};
use crate::token::{Token, TokenType};

pub struct Parser<'src> {
    state: Rc<RefCell<LoxState>>,
    tokens: Vec<Token<'src>>,
    current: usize,
}

macro_rules! rule {
    ($kind:tt$(| $kinds:tt)* => $name:ident($next:ident)) => {
        pub fn $name(&mut self) -> Result<Expr<'src>, ParseError> {
            let mut expr = self.$next()?;

            while self.catch(&[TokenType::$kind$(, TokenType::$kinds)*]) {
                let op = self.previous().clone();
                let rhs = self.$next()?.clone();
                expr = Expr::Binary(op, expr.into(), rhs.into());
            }

            Ok(expr)
        }
    };
}

impl<'src> Parser<'src> {
    pub fn new(state: Rc<RefCell<LoxState>>, tokens: Vec<Token<'src>>) -> Self {
        Parser {
            state,
            tokens,
            current: 0,
        }
    }

    fn previous(&self) -> &Token<'src> {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token<'src> {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    #[inline]
    fn catch(&mut self, kinds: &[TokenType]) -> bool {
        for kind in kinds {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token<'src> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn error(&self, token: &Token<'src>, message: &str) -> ParseError {
        Lox::error_at(self.state.borrow_mut(), token, message);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,

                _ => (),
            }

            self.advance();
        }
    }

    fn consume(&mut self, kind: TokenType, message: &str) -> Result<&Token<'src>, ParseError> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn primary(&mut self) -> Result<Expr<'src>, ParseError> {
        use TokenType as TT;

        if self.catch(&[TT::False]) {
            return Ok(Expr::literal(false));
        }

        if self.catch(&[TT::True]) {
            return Ok(Expr::literal(true));
        }

        if self.catch(&[TT::Nil]) {
            return Ok(Expr::nil());
        }

        if self.catch(&[TT::Number, TT::String]) {
            return Ok(Expr::literal(self.previous().literal.clone()));
        }

        if self.catch(&[TT::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TT::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(expr.into()));
        }

        if self.catch(&[TT::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn unary(&mut self) -> Result<Expr<'src>, ParseError> {
        if self.catch(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let rhs = self.unary()?;

            Ok(Expr::Unary(op, rhs.into()))
        } else {
            self.primary()
        }
    }

    rule!(Slash | Star => factor(unary));
    rule!(Minus | Plus => term(factor));
    rule!(Greater | GreaterEqual | Less | LessEqual => comparison(term));
    rule!(BangEqual | EqualEqual => equality(comparison));

    fn assignment(&mut self) -> Result<Expr<'src>, ParseError> {
        let expr = self.equality()?;

        if self.catch(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = &expr {
                return Ok(Expr::Assign(name.clone(), value.into()));
            }

            self.error(&equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr<'src>, ParseError> {
        self.assignment()
    }

    fn print_statement(&mut self) -> Result<Stmt<'src>, ParseError> {
        let value = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print(value))
    }

    fn block(&mut self) -> Result<Vec<Stmt<'src>>, ParseError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt<'src>, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expr(expr))
    }

    fn statement(&mut self) -> Result<Stmt<'src>, ParseError> {
        if self.catch(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.catch(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }

        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt<'src>, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let initializer = if self.catch(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(name, initializer))
    }

    fn try_declaration(&mut self) -> Result<Stmt<'src>, ParseError> {
        if self.catch(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn declaration(&mut self) -> Stmt<'src> {
        match self.try_declaration() {
            Ok(stmt) => stmt,
            Err(_) => {
                self.synchronize();
                Stmt::Expr(Expr::nil())
            }
        }
    }

    fn try_parse(&mut self) -> Result<Vec<Stmt<'src>>, ParseError> {
        let mut statements = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration());
        }

        Ok(statements)
    }

    pub fn parse(&mut self) -> Vec<Stmt<'src>> {
        self.try_parse().unwrap_or_default()
    }
}
