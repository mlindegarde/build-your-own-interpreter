mod scanner;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};

use crate::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprint!("Failed to read file {}", filename);
        String::new()
    });



    if !file_contents.is_empty() {
        let scanner = Scanner::new(file_contents);
        let tokens = scanner.scan_tokens();

        for el in tokens {
            println!("{}", el)
        }
    } else {
        println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
    }
}