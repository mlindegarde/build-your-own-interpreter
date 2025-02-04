use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u16
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u16) -> Self {
        Token {
            token_type,
            lexeme,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} null", &self.token_type, &self.lexeme)
    }
}