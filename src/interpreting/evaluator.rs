use std::fmt;
use exitcode::ExitCode;
use crate::interpreting::evaluator::EvaluatorResult::{Boolean, Numeric};
use crate::lexing::token::{Token, TokenType};
use crate::lexing::token::TokenType::{Minus, Plus, Slash, Star, Greater, GreaterEqual, Less, LessEqual};
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
            Numeric(value) => write!(f, "{}", value),
            Boolean(value) => write!(f, "{}", value),
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
            "true" | "false" => Ok(Boolean(value == "true")),
            _ => Ok(EvaluatorResult::String(value.to_string()))
        }
    }

    fn numeric_literal(&self, value: f64) -> Result<EvaluatorResult, EvaluationError> {
        Ok(Numeric(value))
    }

    fn is_truthy(result: EvaluatorResult) -> bool {
        match result {
            EvaluatorResult::String(value) => value != "false",
            Numeric(value) => value != 0.0,
            Boolean(value) => value,
            EvaluatorResult::Nil => false,
        }
    }

    fn unary(&self,  operator: &Token, right: &Expression) -> Result<EvaluatorResult, EvaluationError> {
        let right_result = self.evaluate_expression(right)?;

        match (operator.token_type, &right_result) {
            (Minus, Numeric(value)) => Ok(Numeric(-value)),
            (TokenType::Bang, _) => Ok(Boolean(!Self::is_truthy(right_result))),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    fn binary(&self, left: &Expression, operator: &Token, right: &Expression) -> Result<EvaluatorResult, EvaluationError> {
        let left_result = self.evaluate_expression(left)?;
        let right_result = self.evaluate_expression(right)?;

        match (&left_result, &right_result, &operator.token_type) {
            // Numeric operations
            (Numeric(left), Numeric(right), Slash) => Ok(Numeric(left / right)),
            (Numeric(left), Numeric(right), Star) => Ok(Numeric(left * right)),
            (Numeric(left), Numeric(right), Minus) => Ok(Numeric(left - right)),
            (Numeric(left), Numeric(right), Plus) => Ok(Numeric(left + right)),
            (Numeric(left), Numeric(right), Greater) => Ok(Boolean(left > right)),
            (Numeric(left), Numeric(right), GreaterEqual) => Ok(Boolean(left >= right)),
            (Numeric(left), Numeric(right), Less) => Ok(Boolean(left < right)),
            (Numeric(left), Numeric(right), LessEqual) => Ok(Boolean(left <= right)),

            // String operations
            (EvaluatorResult::String(left), EvaluatorResult::String(right), Plus) =>
                Ok(EvaluatorResult::String(format!("{}{}", left, right))),

            // Invalid
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