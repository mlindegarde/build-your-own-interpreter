use crate::lexing::scanning::Scanner;
use crate::lexing::tokenizing::TokenType;

#[test]
fn should_return_eof_token_when_input_is_empty() {
    assert_eq!(
        get_token_types_for(""),
        vec![TokenType::Eof]);
}

#[test]
fn should_return_parenthesis_and_bracket_tokens_when_input_contains_them() {
    assert_eq!(
        get_token_types_for("({})"),
        vec![TokenType::LeftParen, TokenType::LeftBrace,
             TokenType::RightBrace, TokenType::RightParen,
             TokenType::Eof]);
}

#[test]
fn has_error_should_return_true_when_input_contains_errors() {
    let mut scanner = Scanner::new(String::from("|"));
    scanner.scan_tokens();

    assert!(scanner.has_error());
}

#[test]
fn should_return_two_character_operators_when_input_contains_them() {
    assert_eq!(
        get_token_types_for("===<="),
        vec![TokenType::EqualEqual, TokenType::Equal,
             TokenType::LessEqual, TokenType::Eof]);
}


#[test]
fn should_skip_comment_when_input_contains_them() {
    assert_eq!(
        get_token_types_for("()// this is a comment"),
        vec![TokenType::LeftParen, TokenType::RightParen, TokenType::Eof]
    );
}

#[test]
fn should_not_include_comment_value_in_lexeme_if_at_end_of_input() {
    let mut scanner = Scanner::new(String::from("//Comment"));
    let tokens = scanner.scan_tokens();
    let token = tokens.first().unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(token.token_type, TokenType::Eof);
    assert_eq!(token.lexeme, String::from(""));
    assert_eq!(token.line, 1);
}

#[test]
fn should_return_slash_when_not_part_of_comment() {
    assert_eq!(
        get_token_types_for("/*"),
        vec![TokenType::Slash, TokenType::Star, TokenType::Eof]
    )
}

#[test]
fn should_handle_empty_space_when_file_contains_it() {
    let mut scanner = Scanner::new(String::from(" \n\r\t\t(\n(\n"));
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.first().unwrap().lexeme, String::from("("));
    assert_eq!(tokens.iter().nth(1).unwrap().lexeme, String::from("("));
    assert_eq!(tokens.iter().nth(2).unwrap().line, 4);
}

#[test]
fn should_handle_unicode_if_in_input() {
    assert_eq!(
        get_token_types_for("(///Unicode:£§᯽☺♣)"),
        vec![TokenType::LeftParen, TokenType::Eof]
    );
}

fn get_token_types_for(input: &str) -> Vec<TokenType> {
    Scanner::new(String::from(input)).scan_tokens()
        .iter()
        .map(|token| token.token_type)
        .collect()
}