use crate::chunk::{Chunk, OpCode};

impl Chunk {
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

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            let line = self.lines[offset];
            print!("{line:>4} ");
        }

        match self.code[offset].into() {
            OpCode::Constant => Chunk::constant_instruction("OP_CONSTANT", self, offset),
            OpCode::Negate => Chunk::simple_instruction("OP_NEGATE", offset),
            OpCode::Add => Chunk::simple_instruction("OP_ADD", offset),
            OpCode::Subtract => Chunk::simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => Chunk::simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => Chunk::simple_instruction("OP_DIVIDE", offset),
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
