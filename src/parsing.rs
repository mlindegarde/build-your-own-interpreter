use std::fs;
use exitcode::ExitCode;
use crate::lexing::scanning::Scanner;
use crate::parsing::parser::Parser;
use crate::util::error_handling::InterpreterError;

pub mod parser;
pub mod tests;
pub mod consumer;
pub mod expression;

pub fn build_abstract_syntax_tree(filename: &str) -> Result<ExitCode, InterpreterError> {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    println!("{}", ast);
    Ok(exitcode::OK)
}