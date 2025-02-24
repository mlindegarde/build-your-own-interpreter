use crate::lexing::scanning::Scanner;
use crate::lexing::tokenizing::{Token, TokenData, TokenType};
use crate::parsing::parsing::{Expression, Parser};

#[test]
fn should_generate_expected_output() {
    let minus_token = Token::new(1, TokenType::Minus, TokenData::Reserved { lexeme: "-" });
    let star_token = Token::new(1, TokenType::Star, TokenData::Reserved { lexeme: "*" });

    let expression = Expression::Binary {
        left: Box::from(Expression::Unary {
            operator: &minus_token,
            right: Box::from(Expression::StringLiteral { value: "123" })
        }),
        operator: &star_token,
        right: Box::from(Expression::Grouping {
            expression: Box::from(Expression::StringLiteral { value: "45.67" })
        })
    };

    let output = format!("{}", expression);
    assert_eq!(output, "(* (- 123) (group 45.67))");
}

#[test]
fn should_display_38_0() {
    let expression = Expression::NumericLiteral { value: 38.0 };
    let output = format!("{}", expression);

    assert_eq!(output, "38.0");
}

#[test]
fn should_generate_the_correct_ast_for_numeric_comparison() {
    let input = "1 < 3";
    let expected_output = "(< 1.0 3.0)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = &scanner.scan_tokens().unwrap();

    let parser = Parser::new(&tokens);
    let ast = &parser.parse().unwrap();

    let ast_as_string = format!("{}", ast);

    assert_eq!(ast_as_string, expected_output);
}

#[test]
fn should_generate_the_correct_ast_for_string_comparison() {
    let input = "\"a\" < \"b\"";
    let expected_output = "(< a b)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = &scanner.scan_tokens().unwrap();

    let parser = Parser::new(&tokens);
    let ast = &parser.parse().unwrap();

    let ast_as_string = format!("{}", ast);

    assert_eq!(ast_as_string, expected_output);
}

#[test]
fn should_generate_the_correct_ast_given_simple_bang_equals() {
    let input = "\"bar\"!=\"baz\"";
    let expected_output = "(!= bar baz)";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = &scanner.scan_tokens().unwrap();

    let parser = Parser::new(&tokens);
    let ast = &parser.parse().unwrap();

    let ast_as_string = format!("{}", ast);

    assert_eq!(ast_as_string, expected_output);
}

#[test]
fn should_handle_parsing_error() {
    let input = "\"hello";

    let mut scanner = Scanner::new(String::from(input));
    let tokens = &scanner.scan_tokens().unwrap();

    let parser = Parser::new(&tokens);
    let result = &parser.parse();

    assert_eq!(result.is_err(), true);
}
