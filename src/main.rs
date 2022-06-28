extern crate core;

use std::{env, fs, io};
use std::io::{BufRead, Write};

use rustyline::{Editor, Result};
use rustyline::error::ReadlineError;

mod scanner;
mod token;

// TODO: error handling
fn run(source: String) {
    let scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token)
    }
}

const HISTORY_FILE: &str = "repl.history";

fn repl() -> Result<()> {
    let mut rl = Editor::<()>::new();
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.len() == 1 {
                    break;
                }
                println!("Line: {}", line);
                run(line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE)
}

fn run_from_file(filename: &String) {
    let contents = fs::read_to_string(filename);
    run(contents.unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: Graceful handling of command line values
    // First arg is always file name
    if args.len() == 2 {
        run_from_file(&args[1]);
    } else if args.len() > 2 {
        panic!("Usage: rlox [filename]")
    } else {
        repl();
    }
}
