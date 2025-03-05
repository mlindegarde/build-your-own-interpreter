use crate::interpreting::evaluator::Evaluator;
use crate::lexing::scanner::Scanner;
use crate::parsing::parser::Parser;

#[test]
fn should_evaluate_a_simple_group_successfully() {
    let input = "(true)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    let output = evaluator.evaluate().unwrap();

    assert_eq!(output, "true");
}

#[test]
fn should_successfully_negate_value() {
    let input = "-(1)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    let output = evaluator.evaluate().unwrap();

    assert_eq!(output, "-1");
}

#[test]
fn should_successfully_negate_truth_values() {
    let input = "!(false)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    let output = evaluator.evaluate().unwrap();

    assert_eq!(output, "true");
}

#[test]
fn should_successfully_negate_numeric_values() {
    let input = "!10.40";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    let output = evaluator.evaluate().unwrap();

    assert_eq!(output, "false");
}

#[test]
fn should_successfully_negate_nil_values() {
    let input = "!nil";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    let output = evaluator.evaluate().unwrap();

    assert_eq!(output, "true");
}

#[test]
fn should_successfully_do_basic_math() {
    let input = "8 * 4";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    let output = evaluator.evaluate().unwrap();

    assert_eq!(output, "32");
}