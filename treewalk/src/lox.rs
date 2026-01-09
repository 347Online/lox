use std::cell::{RefCell, RefMut};
use std::fmt::Display;
use std::fs::read_to_string;
#[cfg(not(feature = "fancy-repl"))]
use std::io::{Write, stdin, stdout};
use std::rc::Rc;

#[cfg(feature = "fancy-repl")]
use rustyline::DefaultEditor;
#[cfg(feature = "fancy-repl")]
use rustyline::error::ReadlineError;

use crate::error::RuntimeError;
use crate::exit::{RUNTIME_ERROR, SYNTAX_ERROR};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct LoxState {
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl LoxState {
    const fn new() -> Self {
        LoxState {
            had_error: false,
            had_runtime_error: false,
        }
    }
}

#[derive(Debug)]
pub struct Lox {
    state: Rc<RefCell<LoxState>>,
    interpreter: Interpreter,
}

impl<'src> Lox {
    pub fn new() -> Self {
        let state = Rc::new(RefCell::new(LoxState::new()));
        let interpreter = Interpreter::new(state.clone());

        Lox { state, interpreter }
    }

    fn run(&mut self, source: &'src str) {
        let scanner = Scanner::new(self.state.clone(), source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(self.state.clone(), tokens);
        let statements = parser.parse();

        // Stop if there was a syntax error.
        if self.state.borrow().had_error {
            return;
        }

        self.interpreter.interpret(statements);
    }

    pub fn error(state: RefMut<LoxState>, line: usize, message: &str) {
        Lox::report(state, line, "", message);
    }

    pub fn error_at(state: RefMut<LoxState>, token: &Token, message: &str) {
        if token.kind == TokenType::Eof {
            Lox::report(state, token.line, " at end", message);
        } else {
            Lox::report(
                state,
                token.line,
                format!(" at '{}'", token.lexeme),
                message,
            );
        }
    }

    fn report(mut state: RefMut<LoxState>, line: usize, at: impl Display, message: &str) {
        eprintln!("[line {line} ] Error{at}: {message}");
        state.had_error = true;
    }

    #[cfg(feature = "fancy-repl")]
    fn fancy_prompt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut rl = DefaultEditor::new()?;
        let history_path = std::env::home_dir().unwrap().join(".cache/lox_history");

        let _ = rl.load_history(&history_path);

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str())?;
                    self.run(&line);
                    self.state.borrow_mut().had_error = false;
                }
                Err(ReadlineError::Interrupted) => println!("SIGINT"),
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    break;
                }

                x => {
                    x?;
                }
            }
        }

        rl.save_history(&history_path)?;
        Ok(())
    }

    #[cfg(not(feature = "fancy-repl"))]
    fn basic_prompt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut line = String::new();
        let input = stdin();

        loop {
            print!("> ");
            stdout().lock().flush()?;

            line.clear();
            input.read_line(&mut line)?;

            if line.is_empty() {
                println!();
                break;
            }

            self.run(&line);
            self.state.borrow_mut().had_error = false;
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "fancy-repl")]
        {
            self.fancy_prompt()
        }

        #[cfg(not(feature = "fancy-repl"))]
        {
            self.basic_prompt()
        }
    }

    pub fn run_file(&mut self, path: &str) -> std::io::Result<()> {
        let source = read_to_string(path)?;

        self.run(&source);

        if self.state.borrow().had_error {
            std::process::exit(SYNTAX_ERROR);
        }

        if self.state.borrow().had_runtime_error {
            std::process::exit(RUNTIME_ERROR)
        }

        Ok(())
    }

    pub fn runtime_error(mut state: RefMut<LoxState>, err: RuntimeError) {
        eprintln!("{err}");
        state.had_runtime_error = true;
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}
