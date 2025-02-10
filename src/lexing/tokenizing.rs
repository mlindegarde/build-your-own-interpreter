use std::{fmt, fs};
use exitcode::ExitCode;
use crate::lexing::scanning::{ScanningError, Scanner};
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
pub enum TokenData<'a> {
    Reserved { lexeme: &'a str },
    StringLiteral { lexeme: &'a str, literal: &'a str },
    NumericLiteral { lexeme: &'a str, literal: f64 },
    Terminal, Comment
}

//** TOKEN AND TOKEN IMPLEMENTATION ************************************************************************************

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub line: u16,
    pub token_type: TokenType,
    pub token_data: TokenData<'a>
}

impl<'a> Token<'a> {
    pub fn new(line: u16, token_type: TokenType, token: TokenData<'a>) -> Self {
        Token {
            line,
            token_type,
            token_data: token
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token_data {
            TokenData::Reserved { lexeme } => write!(f, "{} {} null", self.token_type, lexeme),
            TokenData::StringLiteral { lexeme, literal } => write!(f, "{} {} {}", self.token_type, lexeme, literal),
            TokenData::NumericLiteral { lexeme, literal } => write!(f, "{} {} {:?}", self.token_type, lexeme, literal),
            TokenData::Terminal | TokenData::Comment => write!(f, "{}  null", self.token_type)
        }
    }
}

//** TOKENIZING COMMAND LOGIC ******************************************************************************************

fn display_tokens(tokens: &[Token]) {
    for token_info in tokens {
        println!("{}", token_info);
    }
}

fn display_errors(errors: &[ScanningError]) {
    for error in errors {
        eprintln!("{}", error);
    }
}

pub fn tokenize_file(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    match Scanner::new(file_contents).scan_tokens() {
        Ok(tokens) => {
            display_tokens(&tokens);
            exitcode::OK
        },
        Err((tokens, errors)) => {
            display_errors(&errors);
            display_tokens(&tokens);
            exitcode::DATAERR
        }
    }
}