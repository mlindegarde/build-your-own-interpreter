use std::fs;
use crate::lexing::scanner::Scanner;
use crate::util::error_handling::InterpreterError;

pub mod consumer;
pub mod token;
pub mod scanner;

mod tests;

pub fn tokenize_file(filename: &str) -> Result<String, InterpreterError>{
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents.clone());
    let tokens = scanner.scan_tokens()?;

    let mut output = Vec::new();

    for token_info in tokens {
        output.push(format!("{}", token_info));
    }

    Ok(output.join("\n"))
}