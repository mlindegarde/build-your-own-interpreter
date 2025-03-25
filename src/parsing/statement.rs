use crate::parsing::expression::Expression;

pub enum Statement {
    PrintStmt { expression: Expression },
    ExpressionStmt { expression: Expression }
}