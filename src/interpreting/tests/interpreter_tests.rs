use crate::interpreting::evaluator::Evaluator;
use crate::interpreting::interpreter::Interpreter;
use crate::lexing::scanner::Scanner;
use crate::parsing::parser::Parser;
use crate::util::error_handling::InterpreterError;

#[test]
fn should_handle_error() {
    let result = interpret("print;");

    assert!(result.is_err());
}

fn interpret(input: &str) -> Result<String, InterpreterError> {
    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let interpreter = Interpreter::new(statements, Evaluator::new(None));
    interpreter.interpret()
}