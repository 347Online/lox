use std::fs::read_to_string;
use std::io::{Write, stdin, stdout};

use common::exit::{IO_ERROR, RUNTIME_ERROR, SYNTAX_ERROR};

use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
use crate::stack::Stack;
use crate::value::Value;

pub const STACK_MAX: usize = 256;

pub enum InterpretError {
    IoError(std::io::Error),
    CompileError,
    RuntimeError,
}

impl From<std::io::Error> for InterpretError {
    fn from(value: std::io::Error) -> Self {
        InterpretError::IoError(value)
    }
}

pub type InterpretResult = Result<(), InterpretError>;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Stack<Value, STACK_MAX>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            chunk: Chunk::new(),
            ip: 0,
            stack: Stack::new(),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop()
    }

    pub fn run(&mut self) -> InterpretResult {
        macro_rules! read_byte {
            () => {{
                let byte = self.chunk.code[self.ip];
                self.ip += 1;
                byte
            }};
        }

        macro_rules! read_constant {
            () => {
                self.chunk.constants[read_byte!() as usize]
            };
        }

        macro_rules! binary_op {
            ($op:tt) => {{
                let b = self.pop();
                let a = self.pop();
                self.push(a $op b);
            }};
        }

        loop {
            let instruction: OpCode = read_byte!().into();

            #[cfg(debug_assertions)]
            {
                print!("          ");
                for slot in self.stack.iter() {
                    print!("[ {slot} ]")
                }
                println!();

                self.chunk.disassemble_instruction(self.ip - 1);
            }

            match instruction {
                OpCode::Constant => {
                    let constant = read_constant!();
                    self.push(constant);
                }
                OpCode::Add => binary_op!(+),
                OpCode::Subtract => binary_op!(-),
                OpCode::Multiply => binary_op!(*),
                OpCode::Divide => binary_op!(/),
                OpCode::Negate => {
                    let value = self.pop();
                    self.push(-value);
                }
                OpCode::Return => {
                    println!("{}", self.pop());

                    return Ok(());
                }
                OpCode::Unknown(_) => unreachable!(),
            }
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        compile(source);

        Ok(())
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

fn read_line(buf: &mut String) {
    if stdin().read_line(buf).is_err() {
        std::process::exit(IO_ERROR);
    }
}

fn prompt() {
    print!("> ");
    if stdout().lock().flush().is_err() {
        std::process::exit(IO_ERROR);
    }
}

pub fn repl() {
    let mut line = String::new();

    loop {
        prompt();

        read_line(&mut line);

        if line.is_empty() {
            println!();
            break;
        }

        let _ = Vm::new().interpret(&line);
        line.clear();
    }
}

pub fn run_file(path: &str) {
    let Ok(source) = read_to_string(path) else {
        eprintln!("Could not read file \"{path}\".");
        std::process::exit(IO_ERROR);
    };

    let error_code = match Vm::new().interpret(&source) {
        Ok(_) => return,

        Err(err) => match err {
            InterpretError::IoError(_) => IO_ERROR,
            InterpretError::CompileError => SYNTAX_ERROR,
            InterpretError::RuntimeError => RUNTIME_ERROR,
        },
    };

    std::process::exit(error_code)
}
