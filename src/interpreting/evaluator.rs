use std::fmt;
use exitcode::ExitCode;
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

    fn string_literal(&self, expression: &Expression) -> Result<String, EvaluationError> {
        match expression {
            Expression::StringLiteral { value } => Ok(String::from(value)),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    fn numeric_literal(&self, expression: &Expression) -> Result<String, EvaluationError> {
        match expression {
            Expression::NumericLiteral { value } => Ok(format!("{}", value)),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    fn grouping(&self, expression: &Expression) -> Result<String, EvaluationError> {
        match expression {
            Expression::Grouping { expression } => self.evaluate_expression(expression),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    /*
    fn unary(&self,  expression: &Expression) -> Result<String, EvaluationError> {
        match expression {
            Expression::Unary { operator, right } => {
                let right = self.evaluate_expression(right)?;
                Ok(format!("{}{}", operator, right))
            },
            _ => Err(EvaluationError::InvalidExpression)
        }
    }
    */

    fn evaluate_expression(&self, expression: &Expression) -> Result<String, EvaluationError> {
        match expression {
            Expression::StringLiteral { value: _ } => self.string_literal(expression),
            Expression::NumericLiteral { value: _ } => self.numeric_literal(expression),
            Expression::Grouping { expression: inner_expression} => self.grouping(inner_expression),
            _ => Err(EvaluationError::InvalidExpression)
        }
    }

    pub fn evaluate(&self) -> Result<String, EvaluationError> {
        let output = self.evaluate_expression(&self.ast)?;
        Ok(output)
    }
}