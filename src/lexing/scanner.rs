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
        let details: Vec<String> = self.errors
            .iter()
            .map(|token| format!("{}", token))
            .chain(
                self.tokens.iter()
                    .map(|error| format!("{}", error))

            )
            .collect();

        write!(f, "{}", details.join("\n"))
    }
}

impl ExitCodeProvider for ScanningErrorSummary {
    fn get_output(&self) -> Option<String> {
        let details: Vec<String> = self.tokens
            .iter()
            .map(|token| format!("{}", token))
            .collect();

        Some(details.join("\n"))
    }

    fn get_error_details(&self) -> Option<String> {
        let details: Vec<String> = self.errors
            .iter()
            .map(|token| format!("{}", token))
            .collect();

        Some(details.join("\n"))
    }
    
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

    fn get_current_lexeme(&self, trim: Trim, consumer: &Consumer) -> &str {
        let start = match trim {
            Trim::None => consumer.start_car,
            Trim::Both => consumer.start_car + 1
        } as usize;

        let end = match trim {
            Trim::None => consumer.current_char,
            Trim::Both => consumer.current_char - 1
        } as usize;

        let start_index = self.source.char_indices().nth(start).map(|(index, _)| index).unwrap_or(0);
        let end_index = self.source.char_indices().nth(end).map(|(index, _)| index).unwrap_or(self.source.len());

        &self.source[start_index .. end_index]
    }

    fn build_token(&self, token_type: TokenType, token_data: TokenData, consumer: &Consumer) -> Token {
        Token::new(consumer.current_line, token_type, token_data)
    }

    fn build_terminal_token(&self, consumer: &Consumer) -> Token {
        self.build_token(TokenType::Eof, TokenData::new_terminal(), consumer)
    }

    fn build_comment_token(&self, consumer: &mut Consumer) -> Token {
        while consumer.peek() != '\n' && !consumer.is_at_end_of_input() {
            consumer.advance();
        }

        self.build_token(TokenType::Comment, TokenData::Comment, consumer)
    }

    fn build_reserved_token(&self, token_type: TokenType, consumer: &Consumer) -> Token {
        self.build_token(
            token_type,
            TokenData::new_reserved(self.get_current_lexeme(Trim::None, consumer)),
            consumer)
    }

    fn build_reserved_token_using_lookahead(
        &self,
        expected_char: char,
        match_token_type: TokenType,
        else_token_type: TokenType,
        consumer: &mut Consumer) -> Token
    {
        let is_match = consumer.match_char(expected_char);
        self.build_reserved_token(if is_match { match_token_type } else { else_token_type }, consumer)
    }

    fn build_string_literal_token(&self, consumer: &mut Consumer) -> Result<Token, ScanningError> {
        while consumer.peek() != '"' && !consumer.is_at_end_of_input() {
            if consumer.peek() == '\n' { consumer.current_line += 1; }
            consumer.advance();
        }

        if consumer.is_at_end_of_input() {
            return Err(ScanningError::UnterminatedString {
                line: consumer.current_line,
                input: self.get_current_lexeme(Trim::None, consumer).to_string()
            });
        }

        consumer.advance();
        Ok(self.build_token(
            TokenType::String,
            TokenData::new_string_literal(
                self.get_current_lexeme(Trim::None, consumer),
                self.get_current_lexeme(Trim::Both, consumer)
            ),
            consumer))
    }

    fn build_numeric_literal_token(&self, consumer: &mut Consumer) -> Token {
        while consumer.peek().is_ascii_digit() { consumer.advance(); }

        if consumer.peek() == '.' && consumer.peek_next().is_ascii_digit() {
            consumer.advance();

            while consumer.peek().is_ascii_digit() { consumer.advance(); }
        }

        let lexeme = self.get_current_lexeme(Trim::None, consumer);
        let literal = lexeme.parse::<f64>().unwrap();

        self.build_token(
            TokenType::Number,
            //TokenData::NumericLiteral { lexeme, literal },
            TokenData::new_numeric_literal(lexeme, literal),
            consumer)
    }

    fn build_keyword_or_identifier_token(&self, consumer: &mut Consumer) -> Token {
        while consumer.peek().is_ascii_alphanumeric() || consumer.peek() == '_' { consumer.advance(); }

        let lexeme = self.get_current_lexeme(Trim::None, consumer);
        let token_type = *self.keyword_map.get(lexeme).unwrap_or(&TokenType::Identifier);

        self.build_reserved_token(token_type, consumer)
    }

    fn build_error(&self, current_char: char, consumer: &Consumer) -> ScanningError {
        ScanningError::UnexpectedCharacter {
            line: consumer.current_line,
            character: current_char }
    }

    fn scan_token(&self, consumer: &mut Consumer) -> Result<Token, ScanningError> {
        let current_char = consumer.advance();

        match current_char {
            '(' => Ok(self.build_reserved_token(TokenType::LeftParen, consumer)),
            ')' => Ok(self.build_reserved_token(TokenType::RightParen, consumer)),
            '{' => Ok(self.build_reserved_token(TokenType::LeftBrace, consumer)),
            '}' => Ok(self.build_reserved_token(TokenType::RightBrace, consumer)),
            ',' => Ok(self.build_reserved_token(TokenType::Comma, consumer)),
            '.' => Ok(self.build_reserved_token(TokenType::Dot, consumer)),
            '-' => Ok(self.build_reserved_token(TokenType::Minus, consumer)),
            '+' => Ok(self.build_reserved_token(TokenType::Plus, consumer)),
            ';' => Ok(self.build_reserved_token(TokenType::Semicolon, consumer)),
            '*' => Ok(self.build_reserved_token(TokenType::Star, consumer)),
            '!' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::BangEqual, TokenType::Bang, consumer)),
            '=' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::EqualEqual, TokenType::Equal, consumer)),
            '<' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::LessEqual, TokenType::Less, consumer)),
            '>' => Ok(self.build_reserved_token_using_lookahead('=', TokenType::GreaterEqual, TokenType::Greater, consumer)),
            '/' if consumer.match_char('/') => Ok(self.build_comment_token(consumer)),
            '/' => Ok(self.build_reserved_token(TokenType::Slash, consumer)),
            ' ' | '\r' | '\t' => Ok(self.build_reserved_token(TokenType::Whitespace, consumer)),
            '\n' => Ok(self.build_reserved_token(TokenType::EndOfLine, consumer)),
            '"' => Ok(self.build_string_literal_token(consumer)?),
            '0' ..= '9' => Ok(self.build_numeric_literal_token(consumer)),
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => Ok(self.build_keyword_or_identifier_token(consumer)),
            _ => Err(self.build_error(current_char, consumer))
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScanningErrorSummary> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<ScanningError> = Vec::new();
        let mut consumer = Consumer::new(&self.source);

        while !consumer.is_at_end_of_input() {
            consumer.start_car = consumer.current_char;

            match self.scan_token(&mut consumer) {
                Ok(token) => match token.token_type {
                    TokenType::Whitespace | TokenType::Comment => {},
                    TokenType::EndOfLine => consumer.current_line += 1,
                    _ => tokens.push(token)
                },
                Err(error) => errors.push(error)
            }
        }

        tokens.push(self.build_terminal_token(&consumer));

        if errors.is_empty() { Ok(tokens) }
        else { Err(ScanningErrorSummary::new(tokens, errors)) }
    }
}