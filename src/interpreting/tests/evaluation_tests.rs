use crate::interpreting::evaluator::Evaluator;
use crate::lexing::scanner::Scanner;
use crate::parsing::parser::Parser;

#[test]
fn should_evaluate_a_simple_group_successfully() {
    assert_eq!(evaluate("(true)"), "true");
}

#[test]
fn should_successfully_negate_value() {
    assert_eq!(evaluate("-(1)"), "-1");
}

#[test]
fn should_successfully_negate_truth_values() {
    assert_eq!(evaluate("!(false)"), "true");
}

#[test]
fn should_successfully_negate_numeric_values() {
    assert_eq!(evaluate("!10.40"), "false");
}

#[test]
fn should_successfully_negate_nil_values() {
    assert_eq!(evaluate("!nil"), "true");
}

#[test]
fn should_successfully_do_basic_math() {
    assert_eq!(evaluate("8 * 4"), "32");
}

#[test]
fn should_successfully_correctly_add_two_numbers() {
    assert_eq!(evaluate("8 + 4"), "12");
}

#[test]
fn should_concatenate_two_string() {
    assert_eq!(evaluate("\"Hello\" + \"World\""), "HelloWorld");
}

#[test]
fn should_successfully_handle_complex_basic_math() {
    assert_eq!(evaluate("63 + 59 - (-(52 - 89))"), "85");
}

fn evaluate(input: &str) -> String {
    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    evaluator.evaluate().unwrap()
}