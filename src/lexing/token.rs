use std::{fmt};
use crate::util::string_util;

//** TOKEN TYPES *******************************************************************************************************

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma,
    Dot,
    Minus, Plus,
    Semicolon,
    Star, Slash,
    Bang, BangEqual,
    Equal, EqualEqual,
    Less, LessEqual,
    Greater, GreaterEqual,
    String,
    Number,
    Eof,
    And, Or,
    If, Else,
    True, False, Nil,
    For, While,
    Class, Fun, Var,
    This, Super,
    Return,
    Print,
    Identifier,
    Whitespace,
    EndOfLine,
    Comment
}

/// Displays the string value for the enum after converting it to upper snake case:
/// ```
/// LeftBrace -> LEFT_BRACE
/// ```
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_type_as_string = format!("{:?}", self);
        write!(f, "{}", string_util::pascal_to_upper_case_snake(&token_type_as_string))
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        // This compares the enum variant, but not the data.  This is more efficient than using
        // string based comparison as it avoids the extra allocation.  I don't really care what
        // the data is for this comparison, just the variant type.
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for TokenType {}

//** TOKEN DATA ENUM **************************************************************************************************

#[derive(Debug, Clone)]
pub enum TokenData {
    Reserved { lexeme: String },
    StringLiteral { lexeme: String, literal: String },
    NumericLiteral { lexeme: String, literal: f64 },
    Terminal, Comment
}

impl TokenData {
    pub fn new_reserved(lexeme: &str) -> Self {
        TokenData::Reserved { lexeme: String::from(lexeme) }
    }

    pub fn new_string_literal(lexeme: &str, literal: &str) -> Self {
        TokenData::StringLiteral { lexeme: String::from(lexeme), literal: String::from(literal) }
    }

    pub fn new_numeric_literal(lexeme: &str, literal: f64) -> Self {
        TokenData::NumericLiteral { lexeme: String::from(lexeme), literal }
    }

    pub fn new_terminal() -> Self {
        TokenData::Terminal
    }
}

//** TOKEN AND TOKEN IMPLEMENTATION ************************************************************************************

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub line: u16,
    pub token_type: TokenType,
    pub token_data: TokenData
}

impl Token {
    pub fn new(line: u16, token_type: TokenType, token: TokenData) -> Self {
        Token {
            line,
            token_type,
            token_data: token
        }
    }

    pub fn get_name(&self) -> String {
        match &self.token_data {
            TokenData::Reserved { lexeme } |
            TokenData::StringLiteral { lexeme, literal: _ } |
            TokenData::NumericLiteral { lexeme, literal: _} => lexeme.to_string(),
            TokenData::Terminal |
            TokenData::Comment => String::new()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token_data {
            TokenData::Reserved { lexeme } => write!(f, "{} {} null", self.token_type, lexeme),
            TokenData::StringLiteral { lexeme, literal } => write!(f, "{} {} {}", self.token_type, lexeme, literal),
            TokenData::NumericLiteral { lexeme, literal } => write!(f, "{} {} {:?}", self.token_type, lexeme, literal),
            TokenData::Terminal | TokenData::Comment => write!(f, "{}  null", self.token_type)
        }
    }
}