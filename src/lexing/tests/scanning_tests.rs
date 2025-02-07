use crate::lexing::scanning::{ Scanner, ScanningError };
use crate::lexing::tokenizing::{ TokenType, Token };

static EMPTY_TOKEN_LIST: Vec<Token> = Vec::new();

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
    let tokens = scanner.scan_tokens().unwrap_or(&EMPTY_TOKEN_LIST);
    let token = tokens.first().unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(token.token_type, TokenType::Eof);
    assert_eq!(token.lexeme, String::from(""));
    assert_eq!(token.line, 1);
}

#[test]
fn should_return_slash_when_not_part_of_comment() {
    assert_eq!(
        get_token_types_from_input("/*"),
        vec![TokenType::Slash, TokenType::Star, TokenType::Eof]
    )
}

#[test]
fn should_handle_empty_space_when_file_contains_it() {
    let mut scanner = Scanner::new(String::from(" \n\r\t\t(\n(\n"));
    let tokens = scanner.scan_tokens().unwrap_or(&EMPTY_TOKEN_LIST);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.first().unwrap().lexeme, String::from("("));
    assert_eq!(tokens.iter().nth(1).unwrap().lexeme, String::from("("));
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

    let tokens = scanner.scan_tokens().unwrap_or(&EMPTY_TOKEN_LIST);
    let token_types = get_token_types_from_tokens(tokens);

    assert_eq!(
        token_types,
        vec![TokenType::String, TokenType::Eof]
    );

    assert!(tokens.first().is_some_and(|token| token.lexeme == String::from("\"Hello, world!\"")));
}

#[test]
fn should_return_errors_when_string_is_missing_quotes() {
    let mut scanner = Scanner::new(String::from("Hello, world!"));
    let result = scanner.scan_tokens();

    assert!(result.is_err());
}

#[test]
fn should_return_unterminated_string_error_when_closing_quote_is_missing() {
    let mut scanner = Scanner::new(String::from("\"Hello, world!"));
    let result = scanner.scan_tokens();
    assert!(result.is_err());

    let errors = result.err().unwrap().1;
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        ScanningError::UnterminatedString { line, input} => {
            assert_eq!(*line, 1);
            assert_eq!(input, "\"Hello, world!");
        },
        _ => panic!("Error should be UnterminatedString")
    };
}


fn get_token_types_from_tokens(input: &Vec<Token>) -> Vec<TokenType> {
    input.iter().map(|token| token.token_type).collect()
}

fn get_token_types_from_input(input: &str) -> Vec<TokenType> {
    Scanner::new(String::from(input)).scan_tokens().unwrap_or(&Vec::new())
    //get_tokens_from_input(input)
        .iter()
        .map(|token| token.token_type)
        .collect()
}

