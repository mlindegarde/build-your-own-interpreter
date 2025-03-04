use crate::lexing::token::{Token, TokenType};
use crate::parsing::parser::ParsingError;

//** CONSUMER **********************************************************************************************************

pub(crate) struct Consumer {
    tokens: Vec<Token>,
    pub current_index: u16
}

#[allow(dead_code)]
impl<'a> Consumer {
    pub(crate) fn new(tokens: &Vec<Token>) -> Self {
        Consumer {
            tokens: tokens.to_vec(),
            current_index: 0
        }
    }

    pub(crate) fn peek(&self) -> &Token {
        &self.tokens[self.current_index as usize]
    }

    pub(crate) fn previous(&self) -> &Token {
        &self.tokens[(self.current_index - 1) as usize]
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub(crate) fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current_index += 1; }

        self.previous()
    }

    pub(crate) fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }

        self.peek().token_type == token_type
    }

    pub(crate) fn match_token_type(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub(crate) fn consume(&mut self, token_type: TokenType, _message: &str) -> Result<&Token,ParsingError> {
        if self.check(token_type) { return Ok(self.advance()); }

        // CodeCrafters does not want to see this output, so for the time being it is commented out.  This also means
        // that the message parameter is never used.  To avoid compiler warnings, it is prefixed with an underscore.
        //self.error(self.peek(), message);
        Err(ParsingError::UnexpectedToken)
    }

    pub(crate) fn error(&self, token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, "at end", message);
        } else {
            self.report(token.line, &format!("at '{}'", token.get_name()), message);
        }
    }

    fn report(&self, line_number: u16, desc: &str, message: &str) {
        println!("[line {}] Desc: {}, Error: {}", line_number, desc, message);
    }

    /*
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon { return; }

            match self.peek().token_type {
                TokenType::Class |
                TokenType::Fun |
                TokenType::Var |
                TokenType::For |
                TokenType::If |
                TokenType::While |
                TokenType::Print |
                TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
    */
}
