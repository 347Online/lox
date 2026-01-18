use bytecode::chunk::{Chunk, OpCode};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_instruction(OpCode::Constant, 123);
    chunk.write_constant(constant, 123);

    chunk.write_instruction(OpCode::Return, 123);

    chunk.disassemble("test chunk");
}
