use std::fs::read_to_string;
use std::io::stdin;

#[derive(Debug)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    fn run(&mut self, source: &str) {
        todo!("execute {source}")
    }

    pub fn run_prompt(&mut self) -> std::io::Result<()> {
        let mut line = String::new();

        stdin().read_line(&mut line)?;

        self.run(&line);

        Ok(())
    }

    pub fn run_file(&mut self, path: &str) -> std::io::Result<()> {
        let source = read_to_string(path)?;

        self.run(&source);

        Ok(())
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}
