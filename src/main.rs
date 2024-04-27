use std::{
    io::{stdin, Read},
    path::Path,
};

use scanner::Scanner;

pub mod scanner;
pub mod tokens;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt()
    }
}

fn run_prompt() {
    let mut input_string = String::new();
    loop {
        input_string.clear();
        std::io::stdin().read_line(&mut input_string).unwrap();

        if input_string.trim() == ":q" {
            std::process::exit(0);
        }

        run(&input_string);
    }
}

fn run_file(path: impl AsRef<Path>) {}

fn run(string: &str) {
    let tokens = Scanner::scan2(string.trim_end().to_owned());

    println!("{:?}", tokens);
}

fn error(line: u32, message: impl Into<String>) {
    eprintln!("[line: {}] Error: {}", line, message.into())
}
