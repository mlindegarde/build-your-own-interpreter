use crate::lexing::scanning::Scanner;
use crate::lexing::tokenizing::{Token, TokenData, TokenType};
use exitcode::ExitCode;
use std::{fmt, fs};
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ParsingError {
    ExpectedExpression,
    UnexpectedToken
}

#[derive(Debug)]
pub struct ParsingErrorDetails {
    
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::ExpectedExpression => write!(f, "Expected expression."),
            ParsingError::UnexpectedToken => write!(f, "Unexpected token.")
        }
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        "desc"
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Unary { operator: Token, right: Box<Expression> },
    StringLiteral { value: String },
    NumericLiteral { value: f64 },
    Grouping { expression: Box<Expression> }
}

impl Expression {
    pub(crate) fn new_binary(left: Expression, operator: Token, right: Expression) -> Self {
        Expression::Binary { left: Box::from(left), operator, right: Box::from(right) }
    }

    pub(crate) fn new_unary(operator: Token, right: Expression) -> Self {
        Expression::Unary { operator, right: Box::from(right) }
    }

    pub(crate) fn new_string_literal(value: &str) -> Self {
        Expression::StringLiteral { value: String::from(value) }
    }

    pub(crate) fn new_numeric_literal(value: f64) -> Self {
        Expression::NumericLiteral { value }
    }

    pub(crate) fn new_grouping(expression: Expression) -> Self {
        Expression::Grouping { expression: Box::from(expression) }
    }
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

impl fmt::Display for Expression {
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


struct Cursor {
    tokens: Vec<Token>,
    pub current_index: u16
}

impl<'a> Cursor {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Cursor {
            tokens: tokens.to_vec(),
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token,ParsingError> {
        if self.check(token_type) { return Ok(self.advance()); }

        self.error(self.peek(), message);
        //Err(ExitCode::from(65));

        Err(ParsingError::UnexpectedToken)
    }

    fn error(&self, token: &Token, message: &str) {
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

pub struct Parser {
    pub tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens
        }
    }

    fn primary(&self, cursor: &mut Cursor) -> Result<Expression, ParsingError> {
        if cursor.match_token_type(vec![TokenType::False]) { return Ok(Expression::new_string_literal("false")) };
        if cursor.match_token_type(vec![TokenType::True]) { return Ok(Expression::new_string_literal("true")) };
        if cursor.match_token_type(vec![TokenType::Nil]) { return Ok(Expression::new_string_literal("nil")) };

        if cursor.match_token_type(vec![TokenType::Number, TokenType::String]) {
            let previous_token = &self.tokens[(cursor.current_index-1) as usize];

            return match &previous_token.token_data {
                //TokenData::StringLiteral { lexeme: _, literal } => Ok(Expression::StringLiteral { value: literal.clone() }),
                TokenData::StringLiteral { lexeme: _, literal } => Ok(Expression::new_string_literal(literal)),
                TokenData::NumericLiteral { lexeme: _, literal } => Ok(Expression::new_numeric_literal(literal.clone())),
                _ => panic!("adf")
            };
        }

        if cursor.match_token_type(vec![TokenType::LeftParen]) {
            let expression = self.expression(cursor)?;

            return match cursor.consume(TokenType::RightParen, "Expect ')' after expression.") {
                Ok(_) => {
                    Ok(Expression::new_grouping(expression))
                },
                Err(_) => {
                    //cursor.error(cursor.peek(), "Expect ')' after expression.");
                    return Err(ParsingError::ExpectedExpression)
                }
            }
        }

        cursor.error(cursor.peek(), "Expect expression.");
        Err(ParsingError::ExpectedExpression)
    }

    fn unary(&self, cursor: &mut Cursor) -> Result<Expression, ParsingError> {
        if cursor.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.unary(cursor)?;

            return Ok(Expression::new_unary(operator.clone(), right));
        }

        self.primary(cursor)
    }

    fn factor(&self, cursor: &mut Cursor) -> Result<Expression, ParsingError> {
        let mut expression = self.unary(cursor)?;

        while cursor.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.unary(cursor)?;
            expression = Expression::new_binary(expression, operator.clone(), right)
        }

        Ok(expression)
    }


    fn term(&self, cursor: &mut Cursor) -> Result<Expression, ParsingError> {
        let mut expression = self.factor(cursor)?;

        while cursor.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.factor(cursor)?;
            expression = Expression::new_binary(expression, operator.clone(), right)
        }

        Ok(expression)
    }

    fn comparison(&self, cursor: &mut Cursor) -> Result<Expression, ParsingError> {
        let mut expression = self.term(cursor)?;
        //let mut expression = Expression::Literal { value: "" };

        while cursor.match_token_type(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.term(cursor)?;
            expression = Expression::new_binary(expression, operator.clone(), right)
        }

        Ok(expression)
    }

    // force tests to run
    fn equality(&self, cursor: &mut Cursor) -> Result<Expression, ParsingError> {
        let mut expression = self.comparison(cursor)?;

        while cursor.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = &self.tokens[(cursor.current_index-1) as usize];
            let right = self.comparison(cursor)?;

            expression = Expression::new_binary(expression, operator.clone(), right)
        }

        Ok(expression)
    }

    fn expression(&self, cursor: &mut Cursor) -> Result<Expression, ParsingError> {
        self.equality(cursor)
    }

    pub fn parse(&self) -> Result<Expression, ParsingError> {
        let mut cursor = Cursor::new(&self.tokens);

        self.expression(&mut cursor)
    }
}

fn handle_parse_results(expression: &Expression) -> ExitCode {
    println!("{}", expression);
    exitcode::OK
}

pub fn build_abstract_syntax_tree(filename: &str) -> Result<ExitCode, Box<dyn Error>> {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens()?;

    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    handle_parse_results(&ast);
    Ok(exitcode::OK)
}