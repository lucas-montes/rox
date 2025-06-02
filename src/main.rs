use std::io::{self, Write};
use std::{fmt::Display, path::PathBuf};

use scanner::Scanner;

mod parser;
mod scanner;
mod syntax_tree;
mod tokens;

struct ErrorReport {
    line: u64,
    msg: String,
}

enum Command<'a> {
    Exit,
    Run(&'a str),
}

impl<'a> Command<'a> {
    fn new(input: &'a str) -> Self {
        let (command, value) = match input.split_once(' ') {
            Some((c, v)) => (c, v),
            None => (input, ""),
        };
        let (command, _) = (command.trim(), value.trim());
        match command {
            "exit" => Self::Exit,
            _ => Self::Run(input),
        }
    }

    fn execute(&self) {
        match self {
            Self::Exit => {
                std::process::exit(0);
            }
            Self::Run(v) => {
                let scan = Scanner::new(v).scan();
                println!("{:?}", scan);
            }
        }
    }
}
impl Display for Command<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Run(c) => {
                write!(f, "{}", c)
            }
            _ => Err(std::fmt::Error),
        }
    }
}

fn interactive() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap(); //The text appears right away without waiting for enter.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        Command::new(&input).execute()
    }
}

fn main() {
    let paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();

    if paths.is_empty() {
        interactive();
    } else {
        println!("{:?}", paths);
    }
}
