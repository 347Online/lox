use bytecode::chunk::{Chunk, OpCode};
use bytecode::vm::Vm;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_instruction(OpCode::Constant, 123);
    chunk.write_constant(constant, 123);

    chunk.write_instruction(OpCode::Return, 123);

    #[cfg(debug_assertions)]
    chunk.disassemble("test chunk");

    let mut vm = Vm::new();
    let _ = vm.interpret(chunk);
}
