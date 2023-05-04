use lox;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    args.next();

    let lox_interpreter = lox::Lox::get_instance();
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if let Some(path) = args.next() {
        lox_interpreter.run_file(path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(64);
        });
    } else {
        lox_interpreter.run_prompt().unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(64);
        });
    }
}
