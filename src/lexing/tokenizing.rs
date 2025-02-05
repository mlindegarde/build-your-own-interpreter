use std::fmt;
use crate::util::string_util;

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_type_as_string = format!("{:?}", self);
        write!(f, "{}", string_util::pascal_to_upper_case_snake(&token_type_as_string))
    }
}

#[cfg(test)]
mod token_type_tests {
    use super::*;

    #[test]
    fn should_convert_enum_name_to_upper_snake_case() {
        let token_type = TokenType::LeftParen;
        let token_type_as_string = format!("{}", token_type);

        assert_eq!(token_type_as_string, "LEFT_PAREN");
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Token {
            token_type,
            lexeme
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} null", &self.token_type, &self.lexeme)
    }
}