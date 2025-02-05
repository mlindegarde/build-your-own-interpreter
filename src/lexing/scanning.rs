use crate::lexing::tokenizing::{Token, TokenType};

pub struct Scanner {
    source: String,
    start_car: u16,
    current_char: u16,
    current_line: u16,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start_car: 0,
            current_char: 0,
            current_line: 1,
            tokens: Vec::new()
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        self.add_token(TokenType::EOF);
        self.tokens
    }

    fn scan_token(&mut self) {
        let current_char = self.advance();

        match current_char {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            _ => {
                eprint!("Unknown character: {}", current_char);
            }
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
                self.source[self.start_car as usize..self.current_char as usize].to_string(),
                self.current_line));

        self.start_car = self.current_char;
    }
}