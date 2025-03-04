use crate::lexing::scanning::{Scanner, ScanningError};
use crate::lexing::tokenizing::{Token, TokenData, TokenType};

#[test]
fn should_return_eof_token_when_input_is_empty() {
    assert_eq!(
        get_token_types_from_input(""),
        vec![TokenType::Eof]);
}

#[test]
fn should_handle_parenthesis_and_bracket_tokens() {
    assert_eq!(
        get_token_types_from_input("({})"),
        vec![TokenType::LeftParen, TokenType::LeftBrace,
             TokenType::RightBrace, TokenType::RightParen,
             TokenType::Eof]);
}

#[test]
fn has_error_should_return_true_when_input_contains_errors() {
    let mut scanner = Scanner::new(String::from("|"));
    let result = scanner.scan_tokens();

    assert!(result.is_err());
}

#[test]
fn should_handle_two_character_operators() {
    assert_eq!(
        get_token_types_from_input("===<="),
        vec![TokenType::EqualEqual, TokenType::Equal,
             TokenType::LessEqual, TokenType::Eof]);
}

#[test]
fn should_skip_comments() {
    assert_eq!(
        get_token_types_from_input("()// this is a comment"),
        vec![TokenType::LeftParen, TokenType::RightParen, TokenType::Eof]
    );
}

#[test]
fn should_not_include_comment_value_in_lexeme_if_at_end_of_input() {
    let mut scanner = Scanner::new(String::from("//Comment"));
    let tokens = scanner.scan_tokens().unwrap_or(Vec::new());
    let token = tokens.first().unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(token.token_type, TokenType::Eof);
    assert_eq!(token.line, 1);
}

#[test]
fn should_return_slash_when_not_part_of_comment() {
    assert_eq!(
        get_token_types_from_input("/*"),
        vec![TokenType::Slash, TokenType::Star, TokenType::Eof])
}

#[test]
fn should_handle_empty_space_when_file_contains_it() {
    let mut scanner = Scanner::new(String::from(" \n\r\t\t(\n(\n"));
    let tokens = scanner.scan_tokens().unwrap_or(Vec::new());
    let TokenData::Reserved { ref lexeme } = tokens.first().unwrap().token_data else { panic!("Token should be Standard")};

    assert_eq!(tokens.len(), 3);
    assert_eq!(lexeme, "(");
    assert_eq!(tokens.iter().nth(2).unwrap().line, 4);
}

#[test]
fn should_handle_unicode_if_in_input() {
    assert_eq!(
        get_token_types_from_input("(///Unicode:£§᯽☺♣)"),
        vec![TokenType::LeftParen, TokenType::Eof]
    );
}

#[test]
fn should_handle_string_literals() {
    let mut scanner = Scanner::new(String::from("\"Hello, world!\""));

    let tokens = scanner.scan_tokens().unwrap_or(Vec::new());
    let token_types = get_token_types_from_tokens(&tokens);

    assert_eq!(
        token_types,
        vec![TokenType::String, TokenType::Eof]
    );

    let TokenData::StringLiteral { ref lexeme, ref literal } = tokens.first().unwrap().token_data
    else { panic!("Token should be Standard")};

    assert_eq!(lexeme, "\"Hello, world!\"");
    assert_eq!(literal, "Hello, world!");
}

#[test]
fn should_return_unterminated_string_error_when_closing_quote_is_missing() {
    let mut scanner = Scanner::new(String::from("\"Hello, world!"));
    let result = scanner.scan_tokens();
    assert!(result.is_err());

    let errors = result.err().unwrap().errors;
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        ScanningError::UnterminatedString { line, input} => {
            assert_eq!(*line, 1);
            assert_eq!(input, "\"Hello, world!");
        },
        _ => panic!("Error should be UnterminatedString")
    };
}

#[test]
fn should_handle_numeric_literal() {
    let mut scanner = Scanner::new(String::from("12.45"));

    let tokens = scanner.scan_tokens().unwrap_or(Vec::new());
    let token_types = get_token_types_from_tokens(&tokens);

    assert_eq!(
        token_types,
        vec![TokenType::Number, TokenType::Eof]
    );

    let token = tokens.first().unwrap();
    let TokenData::NumericLiteral { ref lexeme, literal } = token.token_data
    else { panic!("Token should be Standard")};

    assert_eq!(lexeme, "12.45");
    assert_eq!(literal, 12.45);
    assert_eq!(format!("{}", token), "NUMBER 12.45 12.45");
}

#[test]
fn should_handle_identifiers() {
    assert_eq!(
        get_token_types_from_input("foo bar _hello"),
        vec![TokenType::Identifier, TokenType::Identifier, TokenType::Identifier, TokenType::Eof]);
}

#[test]
fn should_handle_keywords_and_idenfiers() {
    assert_eq!(
        get_token_types_from_input("if (value == true) { return false; }"),
        vec![TokenType::If, TokenType::LeftParen, TokenType::Identifier, TokenType::EqualEqual,
             TokenType::True, TokenType::RightParen, TokenType::LeftBrace, TokenType::Return,
             TokenType::False, TokenType::Semicolon, TokenType::RightBrace, TokenType::Eof]);
}

fn get_token_types_from_tokens(input: &Vec<Token>) -> Vec<TokenType> {
    input.iter().map(|token| token.token_type).collect()
}

fn get_token_types_from_input(input: &str) -> Vec<TokenType> {
    Scanner::new(String::from(input)).scan_tokens().unwrap_or(Vec::new())
    //get_tokens_from_input(input)
        .iter()
        .map(|token| token.token_type)
        .collect()
}

