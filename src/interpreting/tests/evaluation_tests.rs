use crate::interpreting::evaluator::Evaluator;
use crate::lexing::scanner::Scanner;
use crate::parsing::parser::Parser;

#[test]
fn should_blah() {
    let input = "(true)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let evaluator = Evaluator::new(ast);
    let output = evaluator.evaluate().unwrap();

    assert_eq!(output, "true");
}