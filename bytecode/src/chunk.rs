use num_enum::{FromPrimitive, IntoPrimitive};

use crate::value::Value;

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Return,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: Vec<Value>,
}

impl Chunk {
    #[must_use]
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);

        self.constants.len() - 1
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if addr wider than u8.
    pub fn write_constant(&mut self, addr: usize, line: usize) {
        let byte = u8::try_from(addr).unwrap();
        self.write_byte(byte, line);
    }

    pub fn write_instruction(&mut self, instruction: OpCode, line: usize) {
        self.write_byte(instruction.into(), line);
    }

    fn simple_instruction(name: &'static str, offset: usize) -> usize {
        println!("{name}");

        offset + 1
    }

    fn constant_instruction(name: &'static str, chunk: &Chunk, offset: usize) -> usize {
        let constant = chunk.code[offset + 1];
        let value = chunk.constants[constant as usize];
        println!("{name:<16} {constant:>4} '{value}'");

        offset + 2
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            let line = self.lines[offset];
            print!("{line:>4} ");
        }

        match self.code[offset].into() {
            OpCode::Constant => Chunk::constant_instruction("OP_CONSTANT", self, offset),
            OpCode::Return => Chunk::simple_instruction("OP_RETURN", offset),

            OpCode::Unknown(byte) => {
                println!("Unknown opcode {byte}");

                offset + 1
            }
        }
    }

    pub fn disassemble(&self, name: &'static str) {
        println!("== {name} ==");

        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
