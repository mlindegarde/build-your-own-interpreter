use crate::lexing::scanner::Scanner;
use crate::lexing::token::{Token, TokenData, TokenType};
use crate::parsing::parser::{Parser};
use crate::parsing::expression::{Expression};

#[test]
fn should_generate_expected_output() {
    let minus_token = Token::new(1, TokenType::Minus, TokenData::new_reserved("-"));
    let star_token = Token::new(1, TokenType::Star, TokenData::new_reserved("*"));

    let expression = Expression::binary_from(
        Expression::unary_from(
            minus_token,
            Expression::string_literal_from("123")),
        star_token,
        Expression::grouping_from(
            Expression::string_literal_from("45.67")));

    let output = format!("{}", expression);
    assert_eq!(output, "(* (- 123) (group 45.67))");
}

#[test]
fn should_display_38_0() {
    let expression = Expression::numeric_literal_from(38.0);
    let output = format!("{}", expression);

    assert_eq!(output, "38.0");
}

#[test]
fn should_generate_the_correct_ast_for_numeric_comparison() {
    let input = "1 < 3";
    let expected_output = "(< 1.0 3.0)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = &parser.parse_ast().unwrap();

    let ast_as_string = format!("{}", ast);

    assert_eq!(ast_as_string, expected_output);
}

#[test]
fn should_generate_the_correct_ast_for_string_comparison() {
    let input = "\"a\" < \"b\"";
    let expected_output = "(< a b)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = &parser.parse_ast().unwrap();

    let ast_as_string = format!("{}", ast);

    assert_eq!(ast_as_string, expected_output);
}

#[test]
fn should_generate_the_correct_ast_given_simple_bang_equals() {
    let input = "\"bar\"!=\"baz\"";
    let expected_output = "(!= bar baz)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let ast = &parser.parse_ast().unwrap();

    let ast_as_string = format!("{}", ast);

    assert_eq!(ast_as_string, expected_output);
}

#[test]
fn should_handle_parsing_error() {
    let input = "if (bat == frog";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let result = &parser.parse_ast();

    assert_eq!(result.is_err(), true);
}

#[test]
fn should_generate_the_expected_number_of_statements() {
    let input = r##"
        print "Hello, World!";
        print "Goodbye, World!";
        "##;

    let mut scanner = Scanner::new(String::from(input));
    let tokens = scanner.scan_tokens().unwrap();

    let parser = Parser::new(tokens);
    let result = &parser.parse().unwrap();

    assert_eq!(result.len(), 2);
}