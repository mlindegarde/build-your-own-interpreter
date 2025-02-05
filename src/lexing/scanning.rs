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
            self.scan_token();
        }

        self.add_token(TokenType::Eof);
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
            _ => self.handle_error(current_char)
        }
    }

    fn advance(&mut self) -> char {
        let value = self.source.chars().nth(self.current_char as usize).unwrap();
        self.current_char += 1;

        value
    }

    fn is_at_end(&self) -> bool {
        self.current_char >= self.source.len() as u16
    }

    fn add_token(&mut self, token: TokenType) {
        self.tokens.push(
            Token::new(
                token,
                self.source[self.start_car as usize..self.current_char as usize].to_string()));

        self.start_car = self.current_char;
    }

    fn handle_error(&mut self, current_char: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", self.current_line, current_char);
        self.had_error = true;
        self.start_car = self.current_char;
    }
}