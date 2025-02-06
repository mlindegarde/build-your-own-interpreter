use crate::lexing::tokenizing::TokenType;

#[test]
fn should_convert_enum_name_to_upper_snake_case() {
    let token_type = TokenType::LeftParen;
    let token_type_as_string = format!("{}", token_type);

    assert_eq!(token_type_as_string, "LEFT_PAREN");
}