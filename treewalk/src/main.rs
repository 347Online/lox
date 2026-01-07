use treewalk::exit::TOO_MANY_ARGS;
use treewalk::lox::Lox;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let len = args.len();

    if len > 1 {
        eprintln!("Usage: treewalk [script]");
        std::process::exit(TOO_MANY_ARGS);
    }

    let mut lox = Lox::new();

    if len == 1 {
        let path = args.next().unwrap();
        lox.run_file(&path)?;
    } else {
        lox.run_prompt()?;
    }

    Ok(())
}
