use bytecode::vm::{repl, run_file};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: bytecode [path]");
        std::process::exit(64);
    }
}
