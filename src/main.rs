use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut unexpected_char_err = false;

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let mut file_contents_chars = file_contents.chars();
                while let Some(c) = file_contents_chars.next() {
                    match c {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        '{' => println!("LEFT_BRACE {{ null"),
                        '}' => println!("RIGHT_BRACE }} null"),
                        ',' => println!("COMMA , null"),
                        '.' => println!("DOT . null"),
                        '+' => println!("PLUS + null"),
                        '-' => println!("MINUS - null"),
                        ';' => println!("SEMICOLON ; null"),
                        '*' => println!("STAR * null"),
                        '=' => {
                            let mut peekable = file_contents_chars.clone().peekable();
                            if peekable.next() == Some('=') {
                                file_contents_chars.next();
                                println!("EQUAL_EQUAL == null");
                            } else {
                                println!("EQUAL = null");
                            }
                        },
                        _ => {
                            writeln!(io::stderr(), "[line 1] Error: Unexpected character: {}", c).unwrap();
                            unexpected_char_err = true;
                        }
                    }
                }
                println!("EOF  null");
             } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
             }
                if unexpected_char_err == true {
                    process::exit(65);
                }
        },
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
