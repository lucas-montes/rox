use std::io::{self, Write};
use std::{fmt::Display, path::PathBuf};

use rox::{Interpreter, Parser, Scanner};

struct ErrorReport {
    line: u64,
    msg: String,
}

enum Command {
    Exit,
    Run(String),
}

impl Command {
    fn new(input: String) -> Self {
        let (command, value) = match input.split_once(' ') {
            Some((c, v)) => (c, v),
            None => (input.as_str(), ""),
        };
        let (command, _) = (command.trim(), value.trim());
        match command {
            "exit" => Self::Exit,
            _ => Self::Run(input),
        }
    }

    fn execute(self, inter: &mut Interpreter) {
        match self {
            Self::Exit => {
                std::process::exit(0);
            }
            Self::Run(v) => {
                let scan = Scanner::new(&v).scan();
                if let Some(scan_errors) = scan.errors() {
                    eprintln!("error scanning {:?}", &scan_errors);
                    return;
                };
                let parser = Parser::new(scan.tokens());
                if let Some(parse_errors) = parser.errors() {
                    eprintln!("error parsing {:?}", &parse_errors);
                    return;
                };
                let stmts = parser.results();
                for stmt in stmts {
                    if let Err(err) = inter.evaluate(&stmt) {
                        eprintln!("error interpreting {:?}", &err);
                    };
                }
            }
        }
    }
}
impl Display for Command {
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
    let mut inter = Interpreter::default();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap(); //The text appears right away without waiting for enter.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        Command::new(input).execute(&mut inter)
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
