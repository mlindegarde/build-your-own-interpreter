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
            '!' => self.add_token_based_on_lookahead('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_token_based_on_lookahead('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.add_token_based_on_lookahead('=', TokenType::LessEqual, TokenType::Less ),
            '>' => self.add_token_based_on_lookahead('=', TokenType::GreaterEqual, TokenType::Greater),
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

#[cfg(test)]
mod scanning_tests {
    use super::*;

    #[test]
    fn should_return_eof_token_when_file_is_empty() {
        let mut scanner = Scanner::new(String::new());
        let token_types = get_token_types(scanner.scan_tokens());
        let expected_token_types = vec![TokenType::Eof];

        assert_eq!(token_types, expected_token_types);
    }

    #[test]
    fn should_return_parenthesis_and_bracket_tokens_when_file_contains_them() {
        let mut scanner = Scanner::new(String::from("({})"));
        let token_types = get_token_types(scanner.scan_tokens());
        let expected_token_types = vec![
            TokenType::LeftParen, TokenType::LeftBrace,
            TokenType::RightBrace, TokenType::RightParen,
            TokenType::Eof];

        assert_eq!(token_types, expected_token_types);
    }

    #[test]
    fn has_error_should_return_true_when_file_contains_errors() {
        let mut scanner = Scanner::new(String::from("|"));
        scanner.scan_tokens();

        assert!(scanner.has_error());
    }

    #[test]
    fn sould_return_two_character_operators_when_file_contains_them() {
        let mut scanner = Scanner::new(String::from("===<="));
        let token_types = get_token_types(scanner.scan_tokens());
        let expected_token_types = vec![
            TokenType::EqualEqual, TokenType::Equal, TokenType::LessEqual, TokenType::Eof];

        assert_eq!(token_types, expected_token_types);
    }

    fn get_token_types(tokens: &Vec<Token>) -> Vec<TokenType> {
        tokens
            .iter()
            .map(|token| token.token_type)
            .collect()
    }
}