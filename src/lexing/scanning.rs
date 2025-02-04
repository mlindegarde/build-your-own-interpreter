use crate::lexing::tokenizing::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new()
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        self.tokens.push(Token::new(TokenType::EOF, String::new(), 1));
        self.tokens
    }
}