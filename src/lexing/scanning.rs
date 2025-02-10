use std::collections::HashMap;
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

//** SCANNER AND SCANNER IMPLEMENTATION ********************************************************************************

enum Trim {
    None,
    Both
}

struct Cursor {
    start_car: u16,
    current_char: u16,
    current_line: u16
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            start_car: 0,
            current_char: 0,
            current_line: 1
        }
    }
}

pub struct Scanner {
    source: String,
    cursor: Cursor,
    keyword_map: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            cursor: Cursor::new(),
            keyword_map: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While)])
        }
    }

    fn advance(&mut self) -> char {
        let value = self.source.chars().nth(self.cursor.current_char as usize).unwrap();
        self.cursor.current_char += 1;

        value
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end_of_input() {
            return false;
        }

        match self.source.chars().nth(self.cursor.current_char as usize) {
            Some(value) if value == expected => {
                self.cursor.current_char += 1;
                true
            },
            Some(_) | None => false
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end_of_input() {
            '\0'
        } else {
            self.source.chars().nth(self.cursor.current_char as usize).unwrap_or('\0')
        }
    }

    fn peek_next(&self) -> char {
        if self.cursor.current_char + 1 >= self.source.chars().count() as u16 {
            '\0'
        } else {
            self.source.chars().nth((self.cursor.current_char + 1) as usize).unwrap_or('\0')
        }
    }

    fn get_current_lexeme(&self, trim: Trim) -> String {
        let start = match trim {
            Trim::None => self.cursor.start_car,
            Trim::Both => self.cursor.start_car + 1
        } as usize;

        let end = match trim {
            Trim::None => self.cursor.current_char,
            Trim::Both => self.cursor.current_char - 1
        } as usize;

        self.source[start .. end].to_string()
    }

    fn is_at_end_of_input(&self) -> bool {
        // Interesting discovery, self.source.len() assumes 8 bit characters and does not
        // properly count the length in Unicode characters are in the string.
        self.cursor.current_char >= self.source.chars().count() as u16
    }

    fn build_token(&self, token_type: TokenType, token_data: TokenData) -> Token {
        Token::new(self.cursor.current_line, token_type, token_data)
    }

    fn build_terminal_token(&self) -> Token {
        self.build_token(TokenType::Eof, TokenData::Terminal)
    }

    fn build_comment_token(&mut self) -> Token {
        while self.peek() != '\n' && !self.is_at_end_of_input() {
            self.advance();
        }

        self.build_token(TokenType::Comment, TokenData::Comment)
    }

    fn build_reserved_token(&self, token_type: TokenType) -> Token {
        self.build_token(
            token_type,
            TokenData::Reserved { lexeme: self.get_current_lexeme(Trim::None) })
    }

    fn build_reserved_token_using_lookahead(
        &mut self, expected_char: char,
        match_token_type: TokenType,
        else_token_type: TokenType) -> Token
    {
        let is_match = self.match_char(expected_char);
        self.build_reserved_token(if is_match { match_token_type } else { else_token_type })
    }

    fn build_string_literal_token(&mut self) -> Result<Token, ScanningError> {
        while self.peek() != '"' && !self.is_at_end_of_input() {
            if self.peek() == '\n' { self.cursor.current_line += 1; }
            self.advance();
        }

        if self.is_at_end_of_input() {
            return Err(ScanningError::UnterminatedString {
                line: self.cursor.current_line,
                input: self.get_current_lexeme(Trim::None)
            });
        }

        self.advance();
        Ok(self.build_token(
            TokenType::String,
            TokenData::StringLiteral {
                lexeme: self.get_current_lexeme(Trim::None),
                literal: self.get_current_lexeme(Trim::Both)
            }))
    }

    fn build_numeric_literal_token(&mut self) -> Token {
        while self.peek().is_ascii_digit() { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() { self.advance(); }
        }

        let lexeme = self.get_current_lexeme(Trim::None);
        let literal = lexeme.parse::<f64>().unwrap();

        self.build_token(
            TokenType::Number,
            TokenData::NumericLiteral { lexeme, literal })
    }

    fn build_keyword_or_identifier_token(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' { self.advance(); }

        let lexeme = self.get_current_lexeme(Trim::None);
        let token_type = *self.keyword_map.get(&lexeme).unwrap_or(&TokenType::Identifier);

        self.build_reserved_token(token_type)
    }

    fn handle_error(&self, current_char: char) -> ScanningError {
        ScanningError::UnexpectedCharacter {
            line: self.cursor.current_line,
            character: current_char }
    }

    fn scan_token(&mut self, current_char: char) -> Result<Token, ScanningError> {
        match current_char {
            '(' => Ok(self.build_reserved_token(TokenType::LeftParen)),
            ')' => Ok(self.build_reserved_token(TokenType::RightParen)),
            '{' => Ok(self.build_reserved_token(TokenType::LeftBrace)),
            '}' => Ok(self.build_reserved_token(TokenType::RightBrace)),
            ',' => Ok(self.build_reserved_token(TokenType::Comma)),
            '.' => Ok(self.build_reserved_token(TokenType::Dot)),
            '-' => Ok(self.build_reserved_token(TokenType::Minus)),
            '+' => Ok(self.build_reserved_token(TokenType::Plus)),
            ';' => Ok(self.build_reserved_token(TokenType::Semicolon)),
            '*' => Ok(self.build_reserved_token(TokenType::Star)),
            '!' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::BangEqual, TokenType::Bang)),
            '=' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::EqualEqual, TokenType::Equal)),
            '<' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::LessEqual, TokenType::Less)),
            '>' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::GreaterEqual, TokenType::Greater)),
            '/' if self.match_char('/') => Ok(self.build_comment_token()),
            '/' => Ok(self.build_reserved_token(TokenType::Slash)),
            ' ' | '\r' | '\t' => Ok(self.build_reserved_token(TokenType::Whitespace)),
            '\n' => Ok(self.build_reserved_token(TokenType::EndOfLine)),
            '"' => Ok(self.build_string_literal_token()?),
            '0' ..= '9' => Ok(self.build_numeric_literal_token()),
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => Ok(self.build_keyword_or_identifier_token()),
            _ => Err(self.handle_error(current_char))
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, (Vec<Token>, Vec<ScanningError>)> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<ScanningError> = Vec::new();

        while !self.is_at_end_of_input() {
            self.cursor.start_car = self.cursor.current_char;
            let cur_char = self.advance();
            
            match self.scan_token(cur_char) {
                Ok(token) => match token.token_type {
                    TokenType::Whitespace | TokenType::Comment => {},
                    TokenType::EndOfLine => self.cursor.current_line += 1,
                    _ => tokens.push(token)
                },
                Err(error) => {
                    errors.push(error)
                }
            }
        }

        tokens.push(self.build_terminal_token());

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err((tokens, errors))
        }
    }
}