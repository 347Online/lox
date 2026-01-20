#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

pub struct Token<'src> {
    pub kind: TokenType,
    pub lexeme: &'src str,
    pub line: usize,
}

impl<'src> Token<'src> {
    #[must_use]
    pub fn new(kind: TokenType, lexeme: &'src str, line: usize) -> Self {
        Token { kind, lexeme, line }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

pub struct Scanner<'src> {
    // pub source: String,
    pub source: &'src str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'src> Scanner<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        // let source = source.to_owned();

        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, kind: TokenType) -> Token<'src> {
        let lexeme = &self.source[self.start..self.current];
        Token::new(kind, lexeme, self.line)
    }

    fn error_token(&self, message: &'static str) -> Token<'src> {
        Token::new(TokenType::Error, message, self.line)
    }

    fn advance(&mut self) -> char {
        let byte = self.source.as_bytes()[self.current];
        self.current += 1;
        byte as char
    }

    fn catch(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source.as_bytes()[self.current + 1] as char)
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();

            match c {
                c if c.is_ascii_whitespace() => {
                    if c == '\n' {
                        self.line += 1;
                    }

                    self.advance();
                }

                '/' => {
                    if let Some('/') = self.peek_next() {
                        // A comment goes until the end of the line.
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }

                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token<'src> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // The closing quote.
        self.advance();

        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token<'src> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            // Consume the ".".
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'src> {
        use TokenType as TT;

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let name = &self.source[self.start..self.current];

        let kind = match name {
            "and" => TT::And,
            "class" => TT::Class,
            "else" => TT::Else,
            "false" => TT::False,
            "for" => TT::For,
            "fun" => TT::Fun,
            "if" => TT::If,
            "nil" => TT::Nil,
            "or" => TT::Or,
            "print" => TT::Print,
            "return" => TT::Return,
            "super" => TT::Super,
            "this" => TT::This,
            "true" => TT::True,
            "var" => TT::Var,
            "while" => TT::While,

            _ => TT::Identifier,
        };

        self.make_token(kind)
    }

    pub fn scan_token(&mut self) -> Token<'src> {
        use TokenType as TT;

        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        macro_rules! emit_if_else {
            ($c:literal, $a:tt, $b:tt) => {{
                let kind = if self.catch($c) {
                    TokenType::$a
                } else {
                    TokenType::$b
                };

                self.make_token(kind)
            }};
        }

        match c {
            '(' => self.make_token(TT::LeftParen),
            ')' => self.make_token(TT::RightParen),
            '{' => self.make_token(TT::LeftBrace),
            '}' => self.make_token(TT::RightBrace),
            ';' => self.make_token(TT::Semicolon),
            ',' => self.make_token(TT::Comma),
            '.' => self.make_token(TT::Dot),
            '-' => self.make_token(TT::Minus),
            '+' => self.make_token(TT::Plus),
            '/' => self.make_token(TT::Slash),
            '*' => self.make_token(TT::Star),

            '!' => emit_if_else!('=', BangEqual, Bang),
            '=' => emit_if_else!('=', EqualEqual, Equal),
            '<' => emit_if_else!('=', LessEqual, Less),
            '>' => emit_if_else!('=', GreaterEqual, Greater),

            '"' => self.string(),

            c if c.is_ascii_digit() => self.number(),

            c if c.is_ascii_alphanumeric() || c == '_' => self.identifier(),

            _ => self.error_token("Unexpected character."),
        }
    }
}
