use crate::lexing::token::{Token, TokenData, TokenType};
use exitcode::ExitCode;
use std::{fmt};
use crate::parsing::consumer::Consumer;
use crate::parsing::expression::Expression;
use crate::util::error_handling::{ExitCodeProvider};

//** PARSING ERRORS ****************************************************************************************************

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ParsingError {
    ExpectedExpression,
    UnexpectedToken
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::ExpectedExpression => write!(f, "Expected expression."),
            ParsingError::UnexpectedToken => write!(f, "Unexpected token.")
        }
    }
}

impl ExitCodeProvider for ParsingError {
    fn get_exit_code(&self) -> ExitCode {
        match self {
            ParsingError::ExpectedExpression => ExitCode::from(65),
            ParsingError::UnexpectedToken => ExitCode::from(65)
        }
    }
}

//** PARSER ************************************************************************************************************

pub struct Parser {
    pub tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens
        }
    }

    fn primary(&self, consumer: &mut Consumer) -> Result<Expression, ParsingError> {
        if consumer.match_token_type(vec![TokenType::False]) { return Ok(Expression::string_literal_from("false")) };
        if consumer.match_token_type(vec![TokenType::True]) { return Ok(Expression::string_literal_from("true")) };
        if consumer.match_token_type(vec![TokenType::Nil]) { return Ok(Expression::string_literal_from("nil")) };

        if consumer.match_token_type(vec![TokenType::Number, TokenType::String]) {
            let previous_token = &self.tokens[(consumer.current_index-1) as usize];

            return match &previous_token.token_data {
                //TokenData::StringLiteral { lexeme: _, literal } => Ok(Expression::StringLiteral { value: literal.clone() }),
                TokenData::StringLiteral { lexeme: _, literal } => Ok(Expression::string_literal_from(literal)),
                TokenData::NumericLiteral { lexeme: _, literal } => Ok(Expression::numeric_literal_from(literal.clone())),
                _ => panic!("adf")
            };
        }

        if consumer.match_token_type(vec![TokenType::LeftParen]) {
            let expression = self.expression(consumer)?;

            return match consumer.consume(TokenType::RightParen, "Expect ')' after expression.") {
                Ok(_) => {
                    Ok(Expression::grouping_from(expression))
                },
                Err(_) => {
                    //cursor.error(cursor.peek(), "Expect ')' after expression.");
                    return Err(ParsingError::ExpectedExpression)
                }
            }
        }

        consumer.error(consumer.peek(), "Expect expression.");
        Err(ParsingError::ExpectedExpression)
    }

    fn unary(&self, consumer: &mut Consumer) -> Result<Expression, ParsingError> {
        if consumer.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = &self.tokens[(consumer.current_index-1) as usize];
            let right = self.unary(consumer)?;

            return Ok(Expression::unary_from(operator.clone(), right));
        }

        self.primary(consumer)
    }

    fn factor(&self, consumer: &mut Consumer) -> Result<Expression, ParsingError> {
        let mut expression = self.unary(consumer)?;

        while consumer.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = &self.tokens[(consumer.current_index-1) as usize];
            let right = self.unary(consumer)?;
            expression = Expression::binary_from(expression, operator.clone(), right)
        }

        Ok(expression)
    }


    fn term(&self, consumer: &mut Consumer) -> Result<Expression, ParsingError> {
        let mut expression = self.factor(consumer)?;

        while consumer.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = &self.tokens[(consumer.current_index-1) as usize];
            let right = self.factor(consumer)?;
            expression = Expression::binary_from(expression, operator.clone(), right)
        }

        Ok(expression)
    }

    fn comparison(&self, consumer: &mut Consumer) -> Result<Expression, ParsingError> {
        let mut expression = self.term(consumer)?;
        //let mut expression = Expression::Literal { value: "" };

        while consumer.match_token_type(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = &self.tokens[(consumer.current_index-1) as usize];
            let right = self.term(consumer)?;
            expression = Expression::binary_from(expression, operator.clone(), right)
        }

        Ok(expression)
    }

    // force tests to run
    fn equality(&self, consumer: &mut Consumer) -> Result<Expression, ParsingError> {
        let mut expression = self.comparison(consumer)?;

        while consumer.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = &self.tokens[(consumer.current_index-1) as usize];
            let right = self.comparison(consumer)?;

            expression = Expression::binary_from(expression, operator.clone(), right)
        }

        Ok(expression)
    }

    fn expression(&self, consumer: &mut Consumer) -> Result<Expression, ParsingError> {
        self.equality(consumer)
    }

    pub fn parse(&self) -> Result<Expression, ParsingError> {
        let mut consumer = Consumer::new(&self.tokens);

        self.expression(&mut consumer)
    }
}