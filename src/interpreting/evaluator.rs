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

//** EVALUATOR *********************************************************************************************************

pub struct Evaluator {
    pub ast: Expression
}

impl Evaluator {
    pub fn new(ast: Expression) -> Self {
        Self { ast }
    }

    fn string_literal(&self, value: &String) -> Result<String, EvaluationError> {
        Ok(String::from(value))
    }

    fn numeric_literal(&self, value: f64) -> Result<String, EvaluationError> {
        Ok(format!("{}", value))
    }

    fn is_truthy(&self, value: String) -> bool {
        value != "false" && value != "0"
    }

    fn unary(&self,  operator: &Token, right: &Expression) -> Result<String, EvaluationError> {
        match operator.token_type {
            TokenType::Minus => Ok(format!("-{}",  self.evaluate_expression(&right)?)),
            TokenType::Bang => Ok(format!(
                "{}",
                !self.is_truthy(
                    self.evaluate_expression(&right)?))),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    fn evaluate_expression(&self, expression: &Expression) -> Result<String, EvaluationError> {
        match expression {
            Expression::StringLiteral { value } => self.string_literal(value),
            Expression::NumericLiteral { value } => self.numeric_literal(value.clone()),
            Expression::Grouping { expression: inner_expression} => self.evaluate_expression(inner_expression),
            Expression::Unary { operator, right } => self.unary(operator, right),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    pub fn evaluate(&self) -> Result<String, EvaluationError> {
        let output = self.evaluate_expression(&self.ast)?;
        Ok(output)
    }
}