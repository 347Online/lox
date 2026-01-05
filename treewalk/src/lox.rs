use std::fmt::Display;
use std::fs::read_to_string;
use std::io::stdin;

use crate::exit::SYNTAX_ERROR;
use crate::scanner::Scanner;

static mut HAD_ERROR: bool = false;

#[derive(Debug)]
pub struct Lox {
    // had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token:?}");
        }
    }

    pub(crate) fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, at: impl Display, message: &str) {
        eprintln!("[line {line} ] Error{at}: {message}");
        // SAFETY: ???
        unsafe { HAD_ERROR = true };
    }

    pub fn run_prompt(&mut self) -> std::io::Result<()> {
        let input = stdin();
        let mut line = String::new();

        loop {
            print!("> ");
            input.read_line(&mut line)?;

            if line.is_empty() {
                break;
            }

            self.run(&line);
            // SAFETY: ????
            unsafe { HAD_ERROR = false };
        }

        Ok(())
    }

    pub fn run_file(&mut self, path: &str) -> std::io::Result<()> {
        let source = read_to_string(path)?;

        self.run(&source);

        // SAFETY: ???
        if unsafe { HAD_ERROR } {
            std::process::exit(SYNTAX_ERROR);
        }

        Ok(())
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}
