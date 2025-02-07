use std::fmt;

use crate::lexing::tokenizing::{TokenData, Token, TokenType};

//** SCANNING ERRORS. **************************************************************************************************

#[allow(dead_code)]
pub enum ScanningError {
    UnexpectedCharacter { line: u16, character: char },
    UnterminatedString { line: u16, input: String }
}

impl fmt::Display for ScanningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScanningError::UnexpectedCharacter { line, character } => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, character)
            }
            ScanningError::UnterminatedString { line, input: _ } => {
                // The input is known here and I would like to show it, but that wouldn't match
                // the output CodeCrafters expects.
                write!(f, "[line {}] Error: Unterminated string.", line)
            }
        }
    }
}

//* SCANNER AND SCANNER IMPLEMENTATION *********************************************************************************

enum Trim {
    None,
    Both
}

pub struct Scanner {
    source: String,
    start_car: u16,
    current_char: u16,
    current_line: u16,
    tokens: Vec<Token>,
    errors: Vec<ScanningError>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start_car: 0,
            current_char: 0,
            current_line: 1,
            tokens: Vec::new(),
            errors: Vec::new()
        }
    }

    fn advance(&mut self) -> char {
        let value = self.source.chars().nth(self.current_char as usize).unwrap();
        self.current_char += 1;

        value
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end_of_input() {
            return false;
        }

        match self.source.chars().nth(self.current_char as usize) {
            Some(value) if value == expected => {
                self.current_char += 1;
                true
            },
            Some(_) | None => false
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end_of_input() {
            '\0'
        } else {
            self.source.chars().nth(self.current_char as usize).unwrap_or('\0')
        }
    }

    fn get_current_lexeme(&self, trim: Trim) -> String {
        let start = match trim {
            Trim::None => self.start_car,
            Trim::Both => self.start_car + 1
        } as usize;

        let end = match trim {
            Trim::None => self.current_char,
            Trim::Both => self.current_char - 1
        } as usize;

        self.source[start .. end]
            .to_string()
    }
    fn is_at_end_of_input(&self) -> bool {
        // Interesting discovery, self.source.len() assumes 8 bit characters and does not
        // properly count the length in Unicode characters are in the string.
        self.current_char >= self.source.chars().count() as u16
    }

    fn add_terminal_token(&mut self) {
        self.tokens.push(
            Token::new(
                self.current_line,
                TokenType::Eof,
                TokenData::Terminal {}));
    }

    fn add_standard_token(&mut self, token_type: TokenType) {
        self.tokens.push(
            Token::new(
                self.current_line,
                token_type,
                TokenData::Standard { lexeme: self.get_current_lexeme(Trim::None) }));

        self.start_car = self.current_char;
    }

    fn add_standard_token_using_lookahead(&mut self, expected_char: char, match_token_type: TokenType, else_token_type: TokenType) {
        let is_match = self.match_char(expected_char);
        self.add_standard_token(if is_match { match_token_type } else { else_token_type });
    }

    fn add_string_literal_token(&mut self) {
        while self.peek() != '"' && !self.is_at_end_of_input() {
            if self.peek() == '\n' { self.current_line += 1; }
            self.advance();
        }

        if self.is_at_end_of_input() {
            self.errors.push(ScanningError::UnterminatedString {
                line: self.current_line,
                input: self.get_current_lexeme(Trim::None)
            });

            return;
        }

        self.advance();

        self.tokens.push(
            Token::new(
                self.current_line,
                TokenType::String,
                TokenData::StringLiteral {
                    lexeme: self.get_current_lexeme(Trim::None),
                    literal: self.get_current_lexeme(Trim::Both)
                }));

        self.start_car = self.current_char;
    }

    fn handle_error(&mut self, current_char: char) {
        self.errors.push(ScanningError::UnexpectedCharacter {
            line: self.current_line,
            character: current_char });

        self.start_car = self.current_char;
    }

    fn scan_token(&mut self) {
        let current_char = self.advance();

        match current_char {
            '(' => self.add_standard_token(TokenType::LeftParen),
            ')' => self.add_standard_token(TokenType::RightParen),
            '{' => self.add_standard_token(TokenType::LeftBrace),
            '}' => self.add_standard_token(TokenType::RightBrace),
            ',' => self.add_standard_token(TokenType::Comma),
            '.' => self.add_standard_token(TokenType::Dot),
            '-' => self.add_standard_token(TokenType::Minus),
            '+' => self.add_standard_token(TokenType::Plus),
            ';' => self.add_standard_token(TokenType::Semicolon),
            '*' => self.add_standard_token(TokenType::Star),
            '!' => self.add_standard_token_using_lookahead('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_standard_token_using_lookahead('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.add_standard_token_using_lookahead('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.add_standard_token_using_lookahead('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' if self.match_char('/') => while self.peek() != '\n' && !self.is_at_end_of_input() { self.advance(); },
            '/' => self.add_standard_token(TokenType::Slash),
            ' ' | '\r' | '\t' => {},
            '\n' => self.current_line += 1,
            '"' => self.add_string_literal_token(),
            _ => self.handle_error(current_char)
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, (&Vec<Token>, &Vec<ScanningError>)> {
        while !self.is_at_end_of_input() {
            self.start_car = self.current_char;
            self.scan_token();
        }

        self.add_terminal_token();

        if self.errors.is_empty() {
            Ok(&self.tokens)
        } else {
            Err((&self.tokens, &self.errors))
        }
    }
}