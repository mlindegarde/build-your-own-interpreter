use std::{fmt, fs};
use exitcode::ExitCode;
use crate::lexing::scanning::{ScanningError, Scanner};
use crate::util::string_util;

//* TOKEN TYPES ********************************************************************************************************

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
    Eof
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

//* TOKEN AND TOKEN IMPLEMENTATION *************************************************************************************

#[derive(Debug, Clone)]
pub enum Token {
    Standard { lexeme: String, literal: String },
    Terminal {}
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token_type: TokenType,
    pub line: u16,
    pub token: Token
}

impl TokenInfo {
    pub fn new(token_type: TokenType, line: u16, token: Token) -> Self {
        TokenInfo {
            token_type,
            line,
            token
        }
    }
}

fn write_standard_token(f: &mut fmt::Formatter, token_type: TokenType, lexeme: &str, literal: &str) -> core::fmt::Result {
    write!(f, "{} {} {}",
           token_type,
           lexeme,
           match literal {
               Some(literal) => literal,
               None => "null"
           })
}

fn write_terminal_token(f: &mut fmt::Formatter, token_type: TokenType) -> core::fmt::Result {
    write!(f, "{}  null", token_type)
}

impl fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token {
            Token::Standard { lexeme, literal } => write_standard_token(f, self.token_type, lexeme, literal),
            Token::Terminal {} => write!(f, "{}", &self.token_type)
        }
    }
}

//* TOKENIZING COMMAND LOGIC *******************************************************************************************

fn display_tokens(tokens: &[Token]) {
    for token in tokens {
        println!("{}", token);
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
            display_tokens(tokens);
            exitcode::OK
        },
        Err((tokens, errors)) => {
            display_errors(errors);
            display_tokens(tokens);
            exitcode::DATAERR
        }
    }
}