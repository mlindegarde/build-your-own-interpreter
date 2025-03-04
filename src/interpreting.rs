use std::fs;
use exitcode::ExitCode;
use crate::lexing::scanner::Scanner;
use crate::parsing::parser::Parser;
use crate::util::error_handling::InterpreterError;

pub mod evaluator;

pub fn evaluate_ast(filename: &str) -> Result<ExitCode, InterpreterError> {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let evaluator = evaluator::Evaluator::new(ast);
    let result = evaluator.evaluate()?;

    println!("{}", result);
    Ok(exitcode::OK)
}