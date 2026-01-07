use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Expr;
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

    fn error(&mut self, message: &str) -> ParseError {
        Lox::error_at(self.state.borrow_mut(), self.peek(), message);
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

        Err(self.error(message))
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

        Err(self.error("Expect expression."))
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

    fn expression(&mut self) -> Result<Expr<'src>, ParseError> {
        self.equality()
    }

    pub fn parse(&mut self) -> Expr<'src> {
        self.expression().unwrap_or_default()
    }
}
