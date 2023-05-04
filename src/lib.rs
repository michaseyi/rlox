use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process;
pub mod scanner;

static mut GLOBAL_LOX: Lox = Lox::new();

pub struct Lox {
    had_error: bool,
    line: usize,
}

impl Lox {
    const fn new() -> Self {
        Self {
            had_error: false,
            line: 1,
        }
    }

    pub fn get_instance() -> &'static mut Self {
        unsafe { &mut GLOBAL_LOX }
    }

    fn run(&mut self, source: String) {
        let mut scanner = scanner::Scanner::new(source, self.line);
        scanner.scan_tokens();
        for token in scanner.tokens {
            println!("{:?}", token);
        }
        self.line = scanner.line;
    }

    pub fn run_file(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&path)?;
        self.run(content);
        if self.had_error {
            process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Box<dyn Error>> {
        let stdin = stdin();
        let mut stdout = stdout();
        loop {
            print!("> ");
            stdout.flush()?;
            let mut line = String::new();
            stdin.read_line(&mut line)?;
            self.run(line);
            self.had_error = false;
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error {where_}: {message}");
        self.had_error = true;
    }
}
