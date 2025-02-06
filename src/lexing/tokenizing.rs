use std::{fmt, fs};
use crate::lexing::scanning::{ScanningError, Scanner};
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
        // This compares the enum variant, but not the data.  This is more efficient than using
        // string based comparison as it avoids the extra allocation.  I don't really care what
        // the data is for this comparison, just the variant type.
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for TokenType {}

//* TOKEN AND TOKEN IMPLEMENTATION ****************************************************************/

#[allow(dead_code)]
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

pub fn tokenize_file(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    match Scanner::new(file_contents).scan_tokens() {
        Ok(tokens) => display_tokens(tokens),
        Err((tokens, errors)) => {
            display_errors(errors);
            display_tokens(tokens);
            std::process::exit(exitcode::DATAERR);
        }
    }
}