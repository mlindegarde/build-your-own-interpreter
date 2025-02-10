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

    pub fn is_at_end_of_input(&self, source: &str) -> bool {
        self.current_char >= source.chars().count() as u16
    }

    fn peek(&self, source: &str) -> char {
        if self.is_at_end_of_input(source) {
            '\0'
        } else {
            source.chars().nth(self.current_char as usize).unwrap_or('\0')
        }
    }
}

pub struct Scanner {
    source: String,
    keyword_map: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
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

    fn advance(&self, cursor: &mut Cursor) -> char {
        let value = self.source.chars().nth(cursor.current_char as usize).unwrap();
        cursor.current_char += 1;

        value
    }

    fn match_char(&self, expected: char, cursor: &mut Cursor) -> bool {
        if cursor.is_at_end_of_input(&self.source) {
            return false;
        }

        match self.source.chars().nth(cursor.current_char as usize) {
            Some(value) if value == expected => {
                cursor.current_char += 1;
                true
            },
            Some(_) | None => false
        }
    }



    fn peek_next(&self, cursor: &Cursor) -> char {
        if cursor.current_char + 1 >= self.source.chars().count() as u16 {
            '\0'
        } else {
            self.source.chars().nth((cursor.current_char + 1) as usize).unwrap_or('\0')
        }
    }

    fn get_current_lexeme(&self, trim: Trim, cursor: &Cursor) -> &str {
        let start = match trim {
            Trim::None => cursor.start_car,
            Trim::Both => cursor.start_car + 1
        } as usize;

        let end = match trim {
            Trim::None => cursor.current_char,
            Trim::Both => cursor.current_char - 1
        } as usize;

        &self.source[start .. end]
    }

    fn build_token<'a>(&self, token_type: TokenType, token_data: TokenData<'a>, cursor: &Cursor) -> Token<'a> {
        Token::new(cursor.current_line, token_type, token_data)
    }

    fn build_terminal_token(&self, cursor: &Cursor) -> Token {
        self.build_token(TokenType::Eof, TokenData::Terminal, cursor)
    }

    fn build_comment_token(&self, cursor: &mut Cursor) -> Token {
        while cursor.peek(&self.source) != '\n' && !cursor.is_at_end_of_input(&self.source) {
            self.advance(cursor);
        }

        self.build_token(TokenType::Comment, TokenData::Comment, cursor)
    }

    fn build_reserved_token(&self, token_type: TokenType, cursor: &Cursor) -> Token {
        self.build_token(
            token_type,
            TokenData::Reserved { lexeme: self.get_current_lexeme(Trim::None, cursor) },
            cursor)
    }

    fn build_reserved_token_using_lookahead(
        &self, expected_char: char,
        match_token_type: TokenType,
        else_token_type: TokenType,
        cursor: &mut Cursor) -> Token
    {
        let is_match = self.match_char(expected_char, cursor);
        self.build_reserved_token(if is_match { match_token_type } else { else_token_type }, cursor)
    }

    fn build_string_literal_token(&self, cursor: &mut Cursor) -> Result<Token, ScanningError> {
        while cursor.peek(&self.source) != '"' && !cursor.is_at_end_of_input(&self.source) {
            if cursor.peek(&self.source) == '\n' { cursor.current_line += 1; }
            self.advance(cursor);
        }

        if cursor.is_at_end_of_input(&self.source) {
            return Err(ScanningError::UnterminatedString {
                line: cursor.current_line,
                input: self.get_current_lexeme(Trim::None, cursor).to_string()
            });
        }

        self.advance(cursor);
        Ok(self.build_token(
            TokenType::String,
            TokenData::StringLiteral {
                lexeme: self.get_current_lexeme(Trim::None, cursor),
                literal: self.get_current_lexeme(Trim::Both, cursor)
            },
            cursor))
    }

    fn build_numeric_literal_token(&self, cursor: &mut Cursor) -> Token {
        while cursor.peek(&self.source).is_ascii_digit() { self.advance(cursor); }

        if cursor.peek(&self.source) == '.' && self.peek_next(cursor).is_ascii_digit() {
            self.advance(cursor);

            while cursor.peek(&self.source).is_ascii_digit() { self.advance(cursor); }
        }

        let lexeme = self.get_current_lexeme(Trim::None, cursor);
        let literal = lexeme.parse::<f64>().unwrap();

        self.build_token(
            TokenType::Number,
            TokenData::NumericLiteral { lexeme, literal },
            cursor)
    }

    fn build_keyword_or_identifier_token(&self, cursor: &mut Cursor) -> Token {
        while cursor.peek(&self.source).is_ascii_alphanumeric() || cursor.peek(&self.source) == '_' { self.advance(cursor); }

        let lexeme = self.get_current_lexeme(Trim::None, cursor).to_string();
        let token_type = *self.keyword_map.get(&lexeme).unwrap_or(&TokenType::Identifier);

        self.build_reserved_token(token_type, cursor)
    }

    fn handle_error(&self, current_char: char, cursor: &Cursor) -> ScanningError {
        ScanningError::UnexpectedCharacter {
            line: cursor.current_line,
            character: current_char }
    }

    fn scan_token(&self, current_char: char, cursor: &mut Cursor) -> Result<Token, ScanningError> {
        match current_char {
            '(' => Ok(self.build_reserved_token(TokenType::LeftParen, cursor)),
            ')' => Ok(self.build_reserved_token(TokenType::RightParen, cursor)),
            '{' => Ok(self.build_reserved_token(TokenType::LeftBrace, cursor)),
            '}' => Ok(self.build_reserved_token(TokenType::RightBrace, cursor)),
            ',' => Ok(self.build_reserved_token(TokenType::Comma, cursor)),
            '.' => Ok(self.build_reserved_token(TokenType::Dot, cursor)),
            '-' => Ok(self.build_reserved_token(TokenType::Minus, cursor)),
            '+' => Ok(self.build_reserved_token(TokenType::Plus, cursor)),
            ';' => Ok(self.build_reserved_token(TokenType::Semicolon, cursor)),
            '*' => Ok(self.build_reserved_token(TokenType::Star, cursor)),
            '!' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::BangEqual, TokenType::Bang, cursor)),
            '=' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::EqualEqual, TokenType::Equal, cursor)),
            '<' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::LessEqual, TokenType::Less, cursor)),
            '>' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::GreaterEqual, TokenType::Greater, cursor)),
            '/' if self.match_char('/', cursor) => Ok(self.build_comment_token(cursor)),
            '/' => Ok(self.build_reserved_token(TokenType::Slash, cursor)),
            ' ' | '\r' | '\t' => Ok(self.build_reserved_token(TokenType::Whitespace, cursor)),
            '\n' => Ok(self.build_reserved_token(TokenType::EndOfLine, cursor)),
            '"' => Ok(self.build_string_literal_token(cursor)?),
            '0' ..= '9' => Ok(self.build_numeric_literal_token(cursor)),
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => Ok(self.build_keyword_or_identifier_token(cursor)),
            _ => Err(self.handle_error(current_char, cursor))
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, (Vec<Token>, Vec<ScanningError>)> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<ScanningError> = Vec::new();
        let mut cursor = Cursor::new();

        while !cursor.is_at_end_of_input(&self.source) {
            cursor.start_car = cursor.current_char;
            let cur_char = self.advance(&mut cursor);

            match self.scan_token(cur_char, &mut cursor) {
                Ok(token) => match token.token_type {
                    TokenType::Whitespace | TokenType::Comment => {},
                    TokenType::EndOfLine => cursor.current_line += 1,
                    _ => tokens.push(token)
                },
                Err(error) => {
                    errors.push(error)
                }
            }
        }

        tokens.push(self.build_terminal_token(&mut cursor));

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err((tokens, errors))
        }
    }
}