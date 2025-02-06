use std::{fmt, fs};
use crate::lexing::scanning::Scanner;
use crate::util::string_util;

//* TOKEN TYPES ***********************************************************************************/

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
        self.to_string() == other.to_string()
    }
}

impl Eq for TokenType {}

//* TOKEN AND TOKEN IMPLEMENTATION ****************************************************************/

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u16
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u16) -> Self {
        Token {
            token_type,
            lexeme,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} null", &self.token_type, &self.lexeme)
    }
}

//* TOKENIZING COMMAND LOGIC **********************************************************************/

pub fn tokenize(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens();

    for el in tokens {
        println!("{}", el)
    }

    if scanner.has_error() {
        std::process::exit(exitcode::DATAERR);
    }
}