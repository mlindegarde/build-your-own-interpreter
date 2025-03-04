use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug};
use crate::lexing::consumer::{Consumer};
use crate::lexing::token::{TokenData, Token, TokenType};
use crate::util::error_handling::ExitCodeProvider;

//** SCANNING ERRORS ***************************************************************************************************

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
                write!(f, "[line {}] Error: Unterminated string.", line)
            }
        }
    }
}

//** SCANNING ERROR SUMMARY ********************************************************************************************

#[derive(Debug)]
pub struct ScanningErrorSummary {
    pub tokens: Vec<Token>,
    pub errors: Vec<ScanningError>
}

impl<'a> ScanningErrorSummary {
    pub fn new(tokens: Vec<Token>, errors: Vec<ScanningError>) -> Self {
        ScanningErrorSummary {
            tokens,
            errors
        }
    }
}

impl fmt::Display for ScanningErrorSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut details = String::new();

        for error in &self.errors {
            details.push_str(&format!("{}\n", error));
        }

        for token_info in &self.tokens {
            details.push_str(&format!("{}\n", token_info));
        }

        write!(f, "{}", details)
    }
}

impl ExitCodeProvider for ScanningErrorSummary {
    fn get_exit_code(&self) -> i32 {
        exitcode::DATAERR
    }
}

//** SCANNER ************************** ********************************************************************************

enum Trim {
    None,
    Both
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

    fn get_current_lexeme(&self, trim: Trim, caret: &Consumer) -> &str {
        let start = match trim {
            Trim::None => caret.start_car,
            Trim::Both => caret.start_car + 1
        } as usize;

        let end = match trim {
            Trim::None => caret.current_char,
            Trim::Both => caret.current_char - 1
        } as usize;

        &self.source[start .. end]
    }

    fn build_token(&self, token_type: TokenType, token_data: TokenData, caret: &Consumer) -> Token {
        Token::new(caret.current_line, token_type, token_data)
    }

    fn build_terminal_token(&self, caret: &Consumer) -> Token {
        self.build_token(TokenType::Eof, TokenData::new_terminal(), caret)
    }

    fn build_comment_token(&self, caret: &mut Consumer) -> Token {
        while caret.peek() != '\n' && !caret.is_at_end_of_input() {
            caret.advance();
        }

        self.build_token(TokenType::Comment, TokenData::Comment, caret)
    }

    fn build_reserved_token(&self, token_type: TokenType, caret: &Consumer) -> Token {
        self.build_token(
            token_type,
            TokenData::new_reserved(self.get_current_lexeme(Trim::None, caret)),
            caret)
    }

    fn build_reserved_token_using_lookahead(
        &self,
        expected_char: char,
        match_token_type: TokenType,
        else_token_type: TokenType,
        caret: &mut Consumer) -> Token
    {
        let is_match = caret.match_char(expected_char);
        self.build_reserved_token(if is_match { match_token_type } else { else_token_type }, caret)
    }

    fn build_string_literal_token(&self, caret: &mut Consumer) -> Result<Token, ScanningError> {
        while caret.peek() != '"' && !caret.is_at_end_of_input() {
            if caret.peek() == '\n' { caret.current_line += 1; }
            caret.advance();
        }

        if caret.is_at_end_of_input() {
            return Err(ScanningError::UnterminatedString {
                line: caret.current_line,
                input: self.get_current_lexeme(Trim::None, caret).to_string()
            });
        }

        caret.advance();
        Ok(self.build_token(
            TokenType::String,
            TokenData::new_string_literal(
                self.get_current_lexeme(Trim::None, caret),
                self.get_current_lexeme(Trim::Both, caret)
            ),
            caret))
    }

    fn build_numeric_literal_token(&self, caret: &mut Consumer) -> Token {
        while caret.peek().is_ascii_digit() { caret.advance(); }

        if caret.peek() == '.' && caret.peek_next().is_ascii_digit() {
            caret.advance();

            while caret.peek().is_ascii_digit() { caret.advance(); }
        }

        let lexeme = self.get_current_lexeme(Trim::None, caret);
        let literal = lexeme.parse::<f64>().unwrap();

        self.build_token(
            TokenType::Number,
            //TokenData::NumericLiteral { lexeme, literal },
            TokenData::new_numeric_literal(lexeme, literal),
            caret)
    }

    fn build_keyword_or_identifier_token(&self, caret: &mut Consumer) -> Token {
        while caret.peek().is_ascii_alphanumeric() || caret.peek() == '_' { caret.advance(); }

        let lexeme = self.get_current_lexeme(Trim::None, caret);
        let token_type = *self.keyword_map.get(lexeme).unwrap_or(&TokenType::Identifier);

        self.build_reserved_token(token_type, caret)
    }

    fn build_error(&self, current_char: char, caret: &Consumer) -> ScanningError {
        ScanningError::UnexpectedCharacter {
            line: caret.current_line,
            character: current_char }
    }

    fn scan_token(&self, caret: &mut Consumer) -> Result<Token, ScanningError> {
        let current_char = caret.advance();

        match current_char {
            '(' => Ok(self.build_reserved_token(TokenType::LeftParen, caret)),
            ')' => Ok(self.build_reserved_token(TokenType::RightParen, caret)),
            '{' => Ok(self.build_reserved_token(TokenType::LeftBrace, caret)),
            '}' => Ok(self.build_reserved_token(TokenType::RightBrace, caret)),
            ',' => Ok(self.build_reserved_token(TokenType::Comma, caret)),
            '.' => Ok(self.build_reserved_token(TokenType::Dot, caret)),
            '-' => Ok(self.build_reserved_token(TokenType::Minus, caret)),
            '+' => Ok(self.build_reserved_token(TokenType::Plus, caret)),
            ';' => Ok(self.build_reserved_token(TokenType::Semicolon, caret)),
            '*' => Ok(self.build_reserved_token(TokenType::Star, caret)),
            '!' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::BangEqual, TokenType::Bang, caret)),
            '=' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::EqualEqual, TokenType::Equal, caret)),
            '<' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::LessEqual, TokenType::Less, caret)),
            '>' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::GreaterEqual, TokenType::Greater, caret)),
            '/' if caret.match_char('/') => Ok(self.build_comment_token(caret)),
            '/' => Ok(self.build_reserved_token(TokenType::Slash, caret)),
            ' ' | '\r' | '\t' => Ok(self.build_reserved_token(TokenType::Whitespace, caret)),
            '\n' => Ok(self.build_reserved_token(TokenType::EndOfLine, caret)),
            '"' => Ok(self.build_string_literal_token(caret)?),
            '0' ..= '9' => Ok(self.build_numeric_literal_token(caret)),
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => Ok(self.build_keyword_or_identifier_token(caret)),
            _ => Err(self.build_error(current_char, caret))
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScanningErrorSummary> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<ScanningError> = Vec::new();
        let mut caret = Consumer::new(&self.source);

        while !caret.is_at_end_of_input() {
            caret.start_car = caret.current_char;

            match self.scan_token(&mut caret) {
                Ok(token) => match token.token_type {
                    TokenType::Whitespace | TokenType::Comment => {},
                    TokenType::EndOfLine => caret.current_line += 1,
                    _ => tokens.push(token)
                },
                Err(error) => errors.push(error)
            }
        }

        tokens.push(self.build_terminal_token(&caret));

        if errors.is_empty() { Ok(tokens) }
        else { Err(ScanningErrorSummary::new(tokens, errors)) }
    }
}