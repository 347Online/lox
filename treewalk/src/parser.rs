use std::cell::RefCell;
use std::rc::Rc;

use crate::error::ParseError;
use crate::expr::{Expr, ExprData};
use crate::lox::{Lox, LoxState, MAX_ARGS};
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};

pub struct Parser {
    state: Rc<RefCell<LoxState>>,
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! rule {
    ($kind:tt$(| $kinds:tt)* => $name:ident($next:ident) -> $expr:tt) => {
        fn $name(&mut self) -> Result<Expr, ParseError> {
            let mut expr = self.$next()?;

            while self.catch(&[TokenType::$kind$(, TokenType::$kinds)*]) {
                let op = self.previous().clone();
                let rhs = self.$next()?.into();
                expr = Expr::new(ExprData::$expr{ op, lhs: expr.into(), rhs });
            }

            Ok(expr)
        }
    };
    ($kind:tt$(| $kinds:tt)* => $name:ident($next:ident)) => {
        rule!($kind$(| $kinds)* => $name($next) -> Binary);
    };
}

impl Parser {
    pub fn new(state: Rc<RefCell<LoxState>>, tokens: Vec<Token>) -> Self {
        Parser {
            state,
            tokens,
            current: 0,
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
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

    fn catch(&mut self, kinds: &[TokenType]) -> bool {
        for kind in kinds {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
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

    fn consume(&mut self, kind: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
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
            return Ok(Expr::literal(self.previous().literal.as_ref().clone()));
        }

        if self.catch(&[TT::LeftParen]) {
            let expr = self.expression()?.into();
            self.consume(TT::RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::grouping(expr));
        }

        if self.catch(&[TT::Identifier]) {
            let name = self.previous().clone();

            return Ok(Expr::variable(name));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let callee = callee.into();
        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= MAX_ARGS {
                    self.error(
                        self.peek(),
                        &format!("Can't have more than {MAX_ARGS} arguments."),
                    );
                }
                arguments.push(self.expression()?);

                if !self.catch(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self
            .consume(TokenType::RightParen, "Expect ')' after arguments.")?
            .clone();

        Ok(Expr::call(callee, paren, arguments))
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.catch(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.catch(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let rhs = self.unary()?.into();

            Ok(Expr::unary(op, rhs))
        } else {
            self.call()
        }
    }

    rule!(Slash | Star => factor(unary));
    rule!(Minus | Plus => term(factor));
    rule!(Greater | GreaterEqual | Less | LessEqual => comparison(term));
    rule!(BangEqual | EqualEqual => equality(comparison));

    rule!(And => and(equality) -> Logical);
    rule!(Or => or(and) -> Logical);

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.catch(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let ExprData::Variable { name } = &expr.data {
                let name = name.clone();
                let value = value.into();

                return Ok(Expr::assign(name, value));
            }

            self.error(&equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print { expr })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expr { expr })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?.into();
        let else_branch = if self.catch(&[TokenType::Else]) {
            let stmt = self.statement()?.into();
            Some(stmt)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?.into();

        Ok(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.catch(&[TokenType::Semicolon]) {
            None
        } else if self.catch(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let body = self.statement()?;

        let body = match increment {
            Some(increment) => Stmt::Block {
                statements: vec![body, Stmt::Expr { expr: increment }],
            },
            None => body,
        };

        let condition = condition.unwrap_or(Expr::literal(true));
        let body = body.into();
        let body = Stmt::While { condition, body };

        let body = if let Some(initializer) = initializer {
            Stmt::Block {
                statements: vec![initializer, body],
            }
        } else {
            body
        };

        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        let expr = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, expr })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.catch(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.catch(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.catch(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.catch(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.catch(&[TokenType::While]) {
            return self.while_statement();
        };

        if self.catch(&[TokenType::LeftBrace]) {
            let statements = self.block()?;

            return Ok(Stmt::Block { statements });
        }

        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
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

        Ok(Stmt::Var { name, initializer })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {kind} name."))?
            .clone();
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {kind} name."),
        )?;
        let mut parameters = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= MAX_ARGS {
                    self.error(
                        self.peek(),
                        &format!("Can't have more than {MAX_ARGS} parameters."),
                    );
                }

                parameters.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.")?
                        .clone(),
                );

                if !self.catch(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {kind} body."),
        )?;
        let body = self.block()?;

        Ok(Stmt::Function {
            name,
            parameters,
            body,
        })
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = {
            if self.catch(&[TokenType::Fun]) {
                self.function("function")
            } else if self.catch(&[TokenType::Var]) {
                self.var_declaration()
            } else {
                self.statement()
            }
        };

        match result {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        statements
    }
}
