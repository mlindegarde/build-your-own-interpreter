use std::fs;
use crate::interpreting::evaluator::Evaluator;
use crate::interpreting::interpreter::Interpreter;
use crate::lexing::scanner::Scanner;
use crate::parsing::parser::Parser;
use crate::util::error_handling::InterpreterError;

pub mod evaluator;
mod tests;
mod interpreter;

pub fn evaluate_ast(filename: &str) -> Result<String, InterpreterError> {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(tokens);
    let ast = parser.parse_ast()?;

    let evaluator = evaluator::Evaluator::new(Some(ast));
    let result = evaluator.evaluate()?;

    Ok(format!("{}", result))
}

pub fn interpret_program(filename: &str) -> Result<String, InterpreterError> {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let interpreter = Interpreter::new(statements, Evaluator::new(None));
    let result = interpreter.interpret()?;

    Ok(format!("{}", result))
}