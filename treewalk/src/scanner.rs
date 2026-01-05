use crate::lox::Lox;
use crate::token::{Object, Token, TokenType};

pub struct Scanner<'src> {
    source: &'src str,
    tokens: Vec<Token<'src>>,
    start: usize,
    current: usize,
    line: usize,
}

// use TokenType as TT;

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Scanner {
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

    fn add_literal(&mut self, kind: TokenType, literal: Object) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(kind, text, literal, self.line));
    }

    fn add_token(&mut self, kind: TokenType) {
        self.add_literal(kind, Object::Null);
    }

    fn catch(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.char_at(self.current) != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.char_at(self.current))
        }
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

            '\n' => self.line += 1,

            c if c.is_ascii_whitespace() => (),

            _ => Lox::error(self.line, "Unexpected character."),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token<'src>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", Object::Null, self.line));

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
