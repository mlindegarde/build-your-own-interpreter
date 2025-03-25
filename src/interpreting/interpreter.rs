use crate::interpreting::evaluator::Evaluator;
use crate::parsing::statement::Statement;
use crate::util::error_handling::InterpreterError;

pub struct Interpreter {
    pub statements:  Vec<Statement>,
    pub evaluator: Evaluator
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>, evaluator: Evaluator) -> Self {
        Self {
            statements,
            evaluator
        }
    }

    fn execute_statement(&self, statement: &Statement) -> Result<(), InterpreterError> {
        match statement {
            Statement::PrintStmt { expression } => {
                println!("{}", self.evaluator.evaluate_expression(expression)?);
                Ok(())
            },
            Statement::ExpressionStmt { expression } => {
                self.evaluator.evaluate_expression(expression)?;
                Ok(())
            }
        }
    }

    pub fn interpret(&self) -> Result<String, InterpreterError> {
        for statement in &self.statements {
            self.execute_statement(statement)?
        }

        Ok("".to_string())
    }
}