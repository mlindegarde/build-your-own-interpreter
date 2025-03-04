use std::fs;
use exitcode::ExitCode;
use crate::lexing::scanner::Scanner;
use crate::util::error_handling::InterpreterError;

pub mod consumer;
pub mod token;
pub mod scanner;

mod tests;

pub fn tokenize_file(filename: &str) -> Result<ExitCode, InterpreterError>{
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents.clone());
    let tokens = scanner.scan_tokens()?;

    for token_info in tokens {
        println!("{}", token_info);
    }

    Ok(exitcode::OK)
}