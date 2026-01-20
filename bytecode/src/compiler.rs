use crate::scanner::{Scanner, TokenType};

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);

    let mut line = 0;

    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:04} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:02} '{}'", token.kind, token.lexeme);

        if token.kind == TokenType::Eof {
            break;
        }
    }
}
