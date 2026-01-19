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
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
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
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
