use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    #[num_enum(default)]
    Return,
}

pub struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: vec![] }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write(&mut self, instruction: OpCode) {
        self.write_byte(instruction.into());
    }

    fn simple_instruction(name: &'static str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        let byte = self.code[offset];
        if let Ok(instruction) = byte.try_into() {
            match instruction {
                OpCode::Return => Chunk::simple_instruction("OP_RETURN", offset),
            }
        } else {
            println!("Unknown opcode {byte}");
            offset + 1
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
