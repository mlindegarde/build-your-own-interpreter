use std::fmt;
use crate::lexing::token::Token;

//** EXPRESSION ********************************************************************************************************

#[derive(Debug, Clone)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Unary { operator: Token, right: Box<Expression> },
    StringLiteral { value: String },
    NumericLiteral { value: f64 },
    Grouping { expression: Box<Expression> }
}

impl Expression {
    pub(crate) fn binary_from(left: Expression, operator: Token, right: Expression) -> Self {
        Expression::Binary { left: Box::from(left), operator, right: Box::from(right) }
    }

    pub(crate) fn unary_from(operator: Token, right: Expression) -> Self {
        Expression::Unary { operator, right: Box::from(right) }
    }

    pub(crate) fn string_literal_from(value: &str) -> Self {
        Expression::StringLiteral { value: String::from(value) }
    }

    pub(crate) fn numeric_literal_from(value: f64) -> Self {
        Expression::NumericLiteral { value }
    }

    pub(crate) fn grouping_from(expression: Expression) -> Self {
        Expression::Grouping { expression: Box::from(expression) }
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
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Binary { left, operator, right } => {
                write!(f, "{}", Expression::parenthesize(&operator.get_name(), vec![left, right]))
            },
            Expression::Unary { operator, right } => {
                write!(f, "{}", Expression::parenthesize(&operator.get_name(), vec![right]))
            },
            Expression::StringLiteral { value } => write!(f, "{}", value),
            Expression::NumericLiteral { value } => write!(f, "{:?}", value),
            Expression::Grouping { expression } => {
                write!(f, "{}", Expression::parenthesize("group", vec![expression]))
            }
        }
    }
}