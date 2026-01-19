use crate::chunk::{Chunk, OpCode};
use crate::stack::Stack;
use crate::value::Value;

pub const STACK_MAX: usize = 256;

pub enum InterpretError {
    CompileError,
    RuntimeError,
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

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;

        self.run()
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
