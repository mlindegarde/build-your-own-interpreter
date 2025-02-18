use crate::lexing::scanning::Scanner;
use crate::lexing::tokenizing::{Token, TokenData, TokenType};
use exitcode::ExitCode;
use std::{fmt, fs};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Binary { left: Box<Expression<'a>>, operator: &'a Token<'a>, right: Box<Expression<'a>> },
    Unary { operator: &'a Token<'a>, right: Box<Expression<'a>> },
    StringLiteral { value: &'a str },
    NumericLiteral { value: f64 },
    Grouping { expression: Box<Expression<'a>> }
}

fn parenthesize(name: &str, expressions: Vec<&Expression>) -> String {
    let mut output = String::new();

    output.push_str("(");
    output.push_str(name);

    for expression in expressions {
        output.push_str(" ");
        output.push_str(&expression.to_string());
    }

    output.push_str(")");
    output
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Binary { left, operator, right } => {
                write!(f, "{}", parenthesize(&operator.get_name(), vec![left, right]))
            },
            Expression::Unary { operator, right } => {
                write!(f, "{}", parenthesize(&operator.get_name(), vec![right]))
            },
            Expression::StringLiteral { value } => write!(f, "{}", value),
            Expression::NumericLiteral { value } => write!(f, "{:?}", value),
            Expression::Grouping { expression } => {
                write!(f, "{}", parenthesize("group", vec![expression]))
            }
        }
    }
}


struct Cursor<'a> {
    tokens: &'a Vec<Token<'a>>,
    pub current_index: u16
}

impl<'a> Cursor<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Cursor {
            tokens,
            current_index: 0
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_index as usize]
    }

    fn previous(&self) -> &Token {
        &self.tokens[(self.current_index - 1) as usize]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current_index += 1; }

        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }

        self.peek().token_type == token_type
    }

    fn match_token_type(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(token_type) { return self.advance() }

        println!("{}", message);
        self.peek()
    }
}

struct Parser<'a> {
    pub tokens: &'a Vec<Token<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Parser {
            tokens
        }
    }

    fn primary(&self, cursor: &mut Cursor) -> Expression {
        if cursor.match_token_type(vec![TokenType::False]) { return Expression::StringLiteral { value: "false" } };
        if cursor.match_token_type(vec![TokenType::True]) { return Expression::StringLiteral { value: "true" } };
        if cursor.match_token_type(vec![TokenType::Nil]) { return Expression::StringLiteral { value: "nil" } };

        if cursor.match_token_type(vec![TokenType::Number, TokenType::String]) {
            let previous_token = &self.tokens[(cursor.current_index-1) as usize];

            return match previous_token.token_data {
                TokenData::StringLiteral { lexeme: _, literal } => Expression::StringLiteral { value: literal },
                TokenData::NumericLiteral { lexeme: _, literal } => Expression::NumericLiteral { value: literal },
                _ => panic!("adf")
            };
        }

        if cursor.match_token_type(vec![TokenType::LeftParen]) {
            let expression = self.expression(cursor);
            cursor.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expression::Grouping {  expression: Box::from(expression) }
        }

        panic!("");
    }

    fn unary(&self, cursor: &mut Cursor) -> Expression {
        if cursor.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.unary(cursor);

            return Expression::Unary {
                operator,
                right: Box::from(right)
            }
        }

        self.primary(cursor)
    }

    fn factor(&self, cursor: &mut Cursor) -> Expression {
        let mut expression = self.unary(cursor);

        while cursor.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.unary(cursor);
            expression = Expression::Binary {
                left: Box::from(expression),
                operator,
                right: Box::from(right)
            }
        }

        expression
    }


    fn term(&self, cursor: &mut Cursor) -> Expression {
        let mut expression = self.factor(cursor);

        while cursor.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.factor(cursor);
            expression = Expression::Binary {
                left: Box::from(expression),
                operator,
                right: Box::from(right)
            }
        }

        expression
    }


    fn comparison(&self, cursor: &mut Cursor) -> Expression {
        let mut expression = self.term(cursor);
        //let mut expression = Expression::Literal { value: "" };

        while cursor.match_token_type(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.term(cursor);
            //let right = Expression::Literal { value: "" };
            expression = Expression::Binary {
                left: Box::from(expression),
                operator,
                right: Box::from(right)
            }
        }

        expression
    }


    // force tests to run
    fn equality(&self, cursor: &mut Cursor) -> Expression {
        let mut expression = self.comparison(cursor);

        while cursor.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.comparison(cursor);
            let operator = &self.tokens[(cursor.current_index-1) as usize];

            expression = Expression::Binary {
                left: Box::from(expression),
                operator,
                right: Box::from(right)
            }
        }

        expression
    }

    fn expression(&self, cursor: &mut Cursor) -> Expression {
        self.equality(cursor)
    }

    fn parse(&self) -> Expression {
        let mut cursor = Cursor::new(&self.tokens);

        self.expression(&mut cursor)
    }
}

fn handle_parse_results(expression: &Expression) -> ExitCode {
    println!("{}", expression);
    exitcode::OK
}

pub fn build_abstract_syntax_tree(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = &scanner.scan_tokens().unwrap();

    let parser = Parser::new(&tokens);
    let ast = &parser.parse();

    handle_parse_results(&ast)
}