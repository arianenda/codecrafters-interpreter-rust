use std::env;
use std::io::{self, Write};
use std::process::exit;

mod error;
mod expr;
mod identifier;
mod keyword;
mod number;
mod parser;
mod token;
mod scanner;

use parser::{parse, Parser};
use scanner::tokenize;

use crate::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => match tokenize(filename) {
            Ok(tokens) => {
                for token in &tokens {
                    println!("{}", token);
                }
            }
            Err(e) => {
                let e: Error = e.downcast().unwrap();
                exit(e.exit_code as i32);
            }
        },
        "parse" => {
            let tokens = match tokenize(filename) {
                Ok(t) => t,
                Err(e) => {
                    let e: Error = e.downcast().unwrap();
                    exit(e.exit_code as i32);
                }
            };

            match parse(tokens) {
                Some(expr) => println!("{}", expr.print_token_value()),
                None => println!("Error"),
            }
        }
        _ => eprintln!("Unknown command: {}", command),
    }
}
