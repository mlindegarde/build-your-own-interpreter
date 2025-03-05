use std::fmt;
use exitcode::ExitCode;
use crate::lexing::token::{Token, TokenType};
use crate::parsing::expression::Expression;
use crate::util::error_handling::ExitCodeProvider;

//** EVALUATION ERRORS *************************************************************************************************

#[derive(Debug)]
pub enum EvaluationError {
    InvalidExpression
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvaluationError::InvalidExpression => write!(f, "Invalid expression")
        }
    }
}

impl ExitCodeProvider for EvaluationError {
    fn get_output(&self) -> Option<String> {
        None
    }

    fn get_error_details(&self) -> Option<String> {
        Some(format!("{}", self))
    }

    fn get_exit_code(&self) -> ExitCode {
        match self {
            EvaluationError::InvalidExpression => 1
        }
    }
}

//** EVALUATOR RESULT **************************************************************************************************

pub enum EvaluatorResult {
    String(String),
    Numeric(f64),
    Boolean(bool),
    Nil
}

impl fmt::Display for EvaluatorResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvaluatorResult::String(value) => write!(f, "{}", value),
            EvaluatorResult::Numeric(value) => write!(f, "{}", value),
            EvaluatorResult::Boolean(value) => write!(f, "{}", value),
            EvaluatorResult::Nil => write!(f, "nil")
        }
    }
}

//** EVALUATOR *********************************************************************************************************

pub struct Evaluator {
    pub ast: Expression
}

impl Evaluator {
    pub fn new(ast: Expression) -> Self {
        Self { ast }
    }

    fn string_literal(&self, value: &str) -> Result<EvaluatorResult, EvaluationError> {
        match value {
            "nil" => Ok(EvaluatorResult::Nil),
            "true" | "false" => Ok(EvaluatorResult::Boolean(value == "true")),
            _ => Ok(EvaluatorResult::String(value.to_string()))
        }
    }

    fn numeric_literal(&self, value: f64) -> Result<EvaluatorResult, EvaluationError> {
        Ok(EvaluatorResult::Numeric(value))
    }

    fn is_truthy(result: EvaluatorResult) -> bool {
        //value != "false" && value != "0" && value != "nil"

        match result {
            EvaluatorResult::String(value) => value != "false",
            EvaluatorResult::Numeric(value) => value != 0.0,
            EvaluatorResult::Boolean(value) => value,
            EvaluatorResult::Nil => false,
        }
    }

    fn unary(&self,  operator: &Token, right: &Expression) -> Result<EvaluatorResult, EvaluationError> {
        let right_result = self.evaluate_expression(right)?;

        match (operator.token_type, &right_result) {
            (TokenType::Minus, EvaluatorResult::Numeric(value)) => Ok(EvaluatorResult::Numeric(-value)),
            (TokenType::Bang, _) => Ok(EvaluatorResult::Boolean(!Self::is_truthy(right_result))),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    fn get_numeric_values(&self, left: &Expression, right: &Expression) -> (f64, f64) {
        let left_result = self.evaluate_expression(left).unwrap();
        let right_result = self.evaluate_expression(right).unwrap();

        match (left_result, right_result) {
            (EvaluatorResult::Numeric(left_value), EvaluatorResult::Numeric(right_value)) =>
                (left_value, right_value),
            _ => panic!("Invalid numeric values")
        }
    }

    fn divide(&self, left: &Expression, right: &Expression) -> EvaluatorResult {
        let (left, right) = self.get_numeric_values(left, right);
        EvaluatorResult::Numeric(left / right)
    }

    fn multiply(&self, left: &Expression, right: &Expression) -> EvaluatorResult {
        let (left, right) = self.get_numeric_values(left, right);
        EvaluatorResult::Numeric(left * right)
    }

    fn subtract(&self, left: &Expression, right: &Expression) -> EvaluatorResult {
        let (left, right) = self.get_numeric_values(left, right);
        EvaluatorResult::Numeric(left - right)
    }

    fn add(&self, left: &Expression, right: &Expression) -> Result<EvaluatorResult, EvaluationError> {
        let left = self.evaluate_expression(left)?;
        let right = self.evaluate_expression(right)?;

        match (left, right) {
            (EvaluatorResult::Numeric(left), EvaluatorResult::Numeric(right)) =>
                Ok(EvaluatorResult::Numeric(left + right)),
            (EvaluatorResult::String(left), EvaluatorResult::String(right)) =>
                Ok(EvaluatorResult::String(format!("{}{}", left, right))),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    fn binary(&self, left: &Expression, operator: &Token, right: &Expression) -> Result<EvaluatorResult, EvaluationError> {
        match operator.token_type {
            TokenType::Slash => Ok(self.divide(left, right)),
            TokenType::Star => Ok(self.multiply(left, right)),
            TokenType::Minus => Ok(self.subtract(left, right)),
            TokenType::Plus => Ok(self.add(left, right)?),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    fn evaluate_expression(&self, expression: &Expression) -> Result<EvaluatorResult, EvaluationError> {
        match expression {
            Expression::StringLiteral { value } => self.string_literal(value),
            Expression::NumericLiteral { value } => self.numeric_literal(value.clone()),
            Expression::Grouping { expression: inner_expression} => self.evaluate_expression(inner_expression),
            Expression::Unary { operator, right } => self.unary(operator, right),
            Expression::Binary { left, operator, right } => self.binary(left, operator, right)
        }
    }

    pub fn evaluate(&self) -> Result<String, EvaluationError> {
        let output = self.evaluate_expression(&self.ast)?;
        Ok(format!("{}", output))
    }
}