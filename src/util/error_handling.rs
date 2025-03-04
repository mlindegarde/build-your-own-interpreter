use exitcode::ExitCode;
use crate::interpreting::evaluator::EvaluationError;
use crate::lexing::scanner::ScanningErrorSummary;
use crate::parsing::parser::ParsingError;
use crate::ValidationError;

pub trait ExitCodeProvider {
    fn get_output(&self) -> Option<String>;
    fn get_error_details(&self) -> Option<String>;
    fn get_exit_code(&self) -> ExitCode;
}

pub struct InterpreterError {
    pub output: Option<String>,
    pub error_details: Option<String>,
    pub exit_code: ExitCode,
}

impl InterpreterError {
    pub fn new(output: Option<String>, error_details: Option<String>, exit_code: ExitCode) -> Self {
        InterpreterError {
            output,
            error_details,
            exit_code,
        }
    }
}

impl From<ValidationError> for InterpreterError {
    fn from(value: ValidationError) -> Self {
        InterpreterError::new(value.get_output(), value.get_error_details(), value.get_exit_code())
    }
}

impl From<ScanningErrorSummary> for InterpreterError {
    fn from(value: ScanningErrorSummary) -> Self {
        InterpreterError::new(value.get_output(), value.get_error_details(), value.get_exit_code())
    }
}

impl From<ParsingError> for InterpreterError {
    fn from(value: ParsingError) -> Self {
        InterpreterError::new(value.get_output(), value.get_error_details(), value.get_exit_code())
    }
}

impl From<EvaluationError> for InterpreterError {
    fn from(value: EvaluationError) -> Self {
        InterpreterError::new(value.get_output(), value.get_error_details(), value.get_exit_code())
    }
}