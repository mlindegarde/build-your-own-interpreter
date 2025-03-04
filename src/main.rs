extern crate exitcode;
mod lexing;
mod parsing;
mod util;

use exitcode::ExitCode;
use std::fs;
use std::str::FromStr;
use std::{env, fmt};
use std::error::Error;
use crate::lexing::tokenizing::tokenize_file;
use crate::parsing::parsing::build_abstract_syntax_tree;
//** VALIDATION ERRORS *************************************************************************************************

enum ValidationError {
    ArgumentCount { expected: usize, actual: usize },
    Command { provided_command: String },
    Filename { provided_filename: String }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::ArgumentCount { expected, actual } =>
                write!(f, "Expected {} arguments, but received {}.", expected, actual),

            ValidationError::Command { provided_command } =>
                write!(f, "Invalid command: {}", provided_command),

            ValidationError::Filename { provided_filename } =>
                write!(f, "Invalid filename: {}", provided_filename)
        }
    }
}

//** COMMANDS **********************************************************************************************************

enum Command {
    Tokenize,
    Parse
}

/// FromStr does not have a lifetime parameter.  As a result, it can only parse types that
/// do not contain a lifetime parameter.  This means the `ValidationError` enum is less
/// efficient than it could be because it allocates a new String rather than borrowing the
/// existing one.
impl FromStr for Command {
    type Err = ValidationError;

    fn from_str(input: &str) -> Result<Command, ValidationError> {
        match input.to_lowercase().as_str() {
            "tokenize" => Ok(Command::Tokenize),
            "parse" => Ok(Command::Parse),
            _ => Err(ValidationError::Command { provided_command: input.to_string()})
        }
    }
}

fn validate_input(args: &[String]) -> Result<(Command,&String), ValidationError>{
    if args.len() != 3 {
        return Err(ValidationError::ArgumentCount { expected: 3, actual: args.len() });
    }

    // The '?' operator is interesting.  Unpacks the Result if Ok, otherwise it will return
    // the Err.
    let command = Command::from_str(&args[1])?;

    // Nice that pattern matching is a bit more robust than it is in Java.  Closer to what you
    // can do in F#.
    let filename =
        match fs::metadata(&args[2]) {
            Ok(metadata) if metadata.is_file() => &args[2],
            _ => return Err(ValidationError::Filename {
                provided_filename: args[2].to_string()
            })
        };

    Ok((command, filename))
}

//** EXECUTION LOGIC ***************************************************************************************************

fn execute_command(command: &Command, filename: &str) -> Result<ExitCode, Box<dyn Error>> {
    match command {
        Command::Tokenize => tokenize_file(filename),
        Command::Parse => build_abstract_syntax_tree(filename)
    }
}

fn handle_error(error: ValidationError) -> ExitCode {
    eprintln!("{}", error);

    match error {
        ValidationError::ArgumentCount { .. } => exitcode::USAGE,
        ValidationError::Command { .. } => exitcode::USAGE,
        ValidationError::Filename { .. } => exitcode::IOERR
    }
}

fn main() {
    std::process::exit(
        match validate_input(&env::args().collect::<Vec<String>>()) {
            Ok((command, filename)) =>
                match execute_command(&command, filename) {
                    Ok(exit_code) => exit_code,
                    Err(error) => {
                        eprintln!("{}", error);
                        exitcode::USAGE
                    }
                },
            Err(error) => handle_error(error)
        });
}