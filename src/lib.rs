use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        print!("> ");
        stdout.flush()?;
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        run(line);
    }
}

pub fn run_file(path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&path)?;
    run(content);
    Ok(())
}

fn run(source: String) {
    let tokens = source.split_whitespace();

    for token in tokens {
        println!("{token}");
    }
}
