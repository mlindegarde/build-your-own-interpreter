use crate::lexing::tokenizing::{Token, TokenType};

pub struct Scanner {
    source: String,
    start_car: u16,
    current_char: u16,
    current_line: u16,
    had_error: bool,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start_car: 0,
            current_char: 0,
            current_line: 1,
            had_error: false,
            tokens: Vec::new()
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start_car = self.current_char;
            self.scan_token();
        }

        //self.tokens.push(Token::new(TokenType::Eof, String::from(""), self.current_line));
        self.add_token_with_literal(TokenType::Eof, String::new());
        &self.tokens
    }

    pub fn has_error(&self) -> bool {
        self.had_error
    }

    fn scan_token(&mut self) {
        let current_char = self.advance();

        match current_char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token_based_on_lookahead('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_token_based_on_lookahead('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.add_token_based_on_lookahead('=', TokenType::LessEqual, TokenType::Less ),
            '>' => self.add_token_based_on_lookahead('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.current_line += 1,
            _ => self.handle_error(current_char)
        }
    }

    fn add_token_based_on_lookahead(&mut self, expected_char: char, match_token_type: TokenType, else_token_type: TokenType) {
        let is_match = self.match_char(expected_char);
        self.add_token(if is_match { match_token_type } else { else_token_type });
    }

    fn advance(&mut self) -> char {
        let value = self.source.chars().nth(self.current_char as usize).unwrap();
        self.current_char += 1;

        value
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.source.chars().nth(self.current_char as usize) {
            Some(value) =>
                if value == expected {
                    self.current_char += 1;
                    true
                } else { false },
            None => {
                false
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current_char as usize).unwrap_or('\0')
        }
    }

    fn is_at_end(&self) -> bool {
        // Interesting discovery, self.source.len() assumes 8 bit characters and does not
        // properly count the length in Unicode characters are in the string.
        self.current_char >= self.source.chars().count() as u16
    }

    fn add_token(&mut self, token_type: TokenType) {
        /*
        self.tokens.push(
            Token::new(
                token,
                self.source[self.start_car as usize..self.current_char as usize].to_string(),
                self.current_line));

        self.start_car = self.current_char;

         */

        self.add_token_with_literal(
            token_type,
            self.source[self.start_car as usize..self.current_char as usize].to_string()
        )
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, lexeme: String) {
        self.tokens.push(
            Token::new(
                token_type,
                lexeme,
                self.current_line));

        self.start_car = self.current_char;
    }

    fn handle_error(&mut self, current_char: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", self.current_line, current_char);
        self.had_error = true;
        self.start_car = self.current_char;
    }
}