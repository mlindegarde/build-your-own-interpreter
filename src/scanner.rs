use crate::token;
use crate::token::TokenType::EOF;

pub struct Scanner {
    source: String,
    tokens: Vec<token::Token>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new()
        }
    }

    pub fn scan_tokens(mut self) -> Vec<token::Token> {
        self.tokens.push(token::Token::new(EOF, String::new(), 1));
        self.tokens
    }
}
