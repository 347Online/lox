use std::cell::RefCell;
use std::rc::Rc;

use crate::lox::{Lox, LoxState};
use crate::object::Object;
use crate::token::{Token, TokenType};

fn is_identic(c: char, first: bool) -> bool {
    if c == '_' {
        true
    } else if first {
        c.is_ascii_alphabetic()
    } else {
        c.is_alphanumeric()
    }
}

pub struct Scanner<'src> {
    state: Rc<RefCell<LoxState>>,
    source: &'src str,
    tokens: Vec<Token<'src>>,
    start: usize,
    current: usize,
    line: usize,
}

// use TokenType as TT;

impl<'src> Scanner<'src> {
    pub fn new(state: Rc<RefCell<LoxState>>, source: &'src str) -> Self {
        Scanner {
            state,
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn char_at(&self, pos: usize) -> char {
        self.source.as_bytes()[pos..=pos][0] as char
    }

    fn advance(&mut self) -> char {
        let c = self.char_at(self.current);

        self.current += 1;

        c
    }

    fn add_token_literal(&mut self, kind: TokenType, literal: Object) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(kind, text, literal, self.line));
    }

    fn add_token(&mut self, kind: TokenType) {
        self.add_token_literal(kind, Object::Nil);
    }

    fn catch(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.char_at(self.current) != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.char_at(self.current))
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 > self.source.len() {
            None
        } else {
            Some(self.char_at(self.current + 1))
        }
    }

    fn string(&mut self) {
        while let Some(c) = self.peek()
            && c != '"'
        {
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error(self.state.borrow_mut(), self.line, "Unterminated string.");
            return;
        }

        self.advance(); // The closing ".

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::String, Object::String(value.to_string()));
    }

    fn digits(&mut self) {
        while let Some('0'..='9') = self.peek() {
            self.advance();
        }
    }

    fn number(&mut self) {
        self.digits();

        // Look for a fractional part.
        if let Some('.') = self.peek()
            && self.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            // Consume the "."
            self.advance();
            self.digits();
        }

        let x = self.source[self.start..self.current]
            .parse()
            .expect("currently windowed lexeme should always be a valid int or float literal");
        self.add_token_literal(TokenType::Number, Object::Number(x));
    }

    fn identifier(&mut self) {
        while self.peek().is_some_and(|c| is_identic(c, false)) {
            self.advance();
        }

        // match &self.source[self.start..self.current] { _ => self. }
        let kind = match &self.source[self.start..self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,

            _ => TokenType::Identifier,
        };

        self.add_token(kind);
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        macro_rules! emit_token {
            ($kind:tt) => {
                self.add_token(TokenType::$kind)
            };

            ($expected:literal => $yes:tt else $no:tt) => {{
                let kind = if self.catch($expected) {
                    TokenType::$yes
                } else {
                    TokenType::$no
                };

                self.add_token(kind)
            }};

            ($expected:literal => $yes:expr, else $no:tt) => {{
                let kind = if self.catch($expected) {
                    $yes
                    return;
                } else {
                    TokenType::$no
                };

                self.add_token(kind)
            }};
        }

        match c {
            '(' => emit_token!(LeftParen),
            ')' => emit_token!(RightParen),
            '{' => emit_token!(LeftBrace),
            '}' => emit_token!(RightBrace),
            ',' => emit_token!(Comma),
            '.' => emit_token!(Dot),
            '-' => emit_token!(Minus),
            '+' => emit_token!(Plus),
            ';' => emit_token!(Semicolon),
            '*' => emit_token!(Star),

            '!' => emit_token!('=' => BangEqual else Bang),
            '=' => emit_token!('=' => EqualEqual else Equal),
            '<' => emit_token!('=' => LessEqual else Less),
            '>' => emit_token!('=' => GreaterEqual else Greater),

            '/' => {
                if self.catch('/') {
                    // A comment runs until the end of the line.
                    while self.peek().is_some_and(|c| c != '\n') {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            '"' => self.string(),

            c if c.is_ascii_digit() => self.number(),

            c if is_identic(c, true) => self.identifier(),

            // Whitespace
            '\n' => self.line += 1,
            c if c.is_ascii_whitespace() => (),

            _ => Lox::error(self.state.borrow_mut(), self.line, "Unexpected character."),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token<'src>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", Object::Nil, self.line));

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
