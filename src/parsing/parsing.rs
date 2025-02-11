use std::{fmt, fs};
use exitcode::ExitCode;
use crate::lexing::scanning::Scanner;
use crate::lexing::tokenizing;
use crate::lexing::tokenizing::{Token, TokenData, TokenType};

pub enum Expression<'a> {
    Binary { left: Box<Expression<'a>>, operator: &'a Token<'a>, right: Box<Expression<'a>> },
    Unary { operator: &'a Token<'a>, right: Box<Expression<'a>> },
    Literal { value: &'a str },
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
        match &self {
            Expression::Binary { left, operator, right } => {
                write!(f, "{}", parenthesize(&operator.get_name(), vec![left, right]))
            },
            Expression::Unary { operator, right } => {
                write!(f, "{}", parenthesize(&operator.get_name(), vec![right]))
            },
            Expression::Literal { value } => {
                write!(f, "{}", value)
            },
            Expression::Grouping { expression } => {
                write!(f, "{}", parenthesize("group", vec![expression]))
            }
        }
    }
}

pub fn parse_file(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);

    let tokens = scanner.scan_tokens().unwrap_or_else(|(tokens, errors)| {
        tokenizing::display_errors(&errors);
        tokenizing::display_tokens(&tokens);

        Vec::new()
    });

    let minus_token = Token::new(1, TokenType::Minus, TokenData::Reserved { lexeme: "-" });
    let star_token = Token::new(1, TokenType::Star, TokenData::Reserved { lexeme: "*" });

    let expression = Expression::Binary {
        left: Box::from(Expression::Unary {
            operator: &minus_token,
            right: Box::from(Expression::Literal { value: "123" })
        }),
        operator: &star_token,
        right: Box::from(Expression::Grouping {
            expression: Box::from(Expression::Literal { value: "45.67" })
        })
    };

    println!("{}", expression);

    if tokens.len() != 0 { exitcode::OK }
    else { exitcode::DATAERR }
}