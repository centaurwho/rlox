extern crate core;

use std::{env, fs, io};
use std::io::{BufRead, Write};

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

fn repl() {
    let stdin = io::stdin();
    // TODO: Consider using a REPL crate
    loop {
        let mut line = String::new();
        print!(">>> ");
        io::stdout().flush();
        match stdin.lock().read_line(&mut line) {
            Ok(size) => {
                // A way to exit REPL without resorting to CTRL+C
                if size == 1 {
                    break;
                }
            }
            Err(_) => {}
        }
        run(line);
        // println!("line: {}", line.trim().len());
    }
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
