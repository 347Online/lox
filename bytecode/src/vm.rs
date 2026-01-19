use crate::chunk::{Chunk, OpCode};

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub type InterpretResult = Result<(), InterpretError>;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            chunk: Chunk::new(),
            ip: 0,
        }
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

        loop {
            let instruction: OpCode = read_byte!().into();

            #[cfg(feature = "debug_trace_execution")]
            self.chunk.disassemble_instruction(self.ip);

            match instruction {
                OpCode::Constant => {
                    let constant = read_constant!();
                    println!("{constant}");
                }
                OpCode::Return => return Ok(()),

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
