use std::fmt;
use exitcode::ExitCode;
use crate::lexing::scanner::ScanningErrorSummary;
use crate::parsing::parser::ParsingError;
use crate::ValidationError;

pub trait ExitCodeProvider {
    fn get_exit_code(&self) -> ExitCode;
}

pub struct InterpreterError {
    pub details: String,
    pub exit_code: ExitCode,
}

impl InterpreterError {
    pub fn new(details: String, exit_code: ExitCode) -> Self {
        InterpreterError {
            details,
            exit_code,
        }
    }
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<ValidationError> for InterpreterError {
    fn from(value: ValidationError) -> Self {
        InterpreterError::new(value.to_string(), value.get_exit_code())
    }
}

impl From<ScanningErrorSummary> for InterpreterError {
    fn from(value: ScanningErrorSummary) -> Self {
        InterpreterError::new(value.to_string(), value.get_exit_code())
    }
}

impl From<ParsingError> for InterpreterError {
    fn from(value: ParsingError) -> Self {
        InterpreterError::new(value.to_string(), value.get_exit_code())
    }
}