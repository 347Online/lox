use bytecode::chunk::{Chunk, OpCode};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);
    chunk.disassemble("test chunk");
}
