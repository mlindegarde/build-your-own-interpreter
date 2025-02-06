mod lexing;
mod util;

use std::{env, fmt};
use std::fs;
use std::str::FromStr;
use crate::lexing::scanning::Scanner;
use crate::ValidationError::InvalidCommand;
extern crate exitcode;

enum ValidationError {
    InvalidArgumentCount { expected: usize, actual: usize },
    InvalidCommand { provided_command: String },
    InvalidFilename { provided_filename: String }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::InvalidArgumentCount { expected, actual } =>
                write!(f, "Expected {} arguments, but received {}.", expected, actual),

            ValidationError::InvalidCommand { provided_command } =>
                write!(f, "Invalid command: {}", provided_command),

            ValidationError::InvalidFilename { provided_filename } =>
                write!(f, "Invalid filename: {}", provided_filename)
        }
    }
}

enum Command {
    Tokenize
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
            _ => Err(InvalidCommand { provided_command: input.to_string()})
        }
    }
}

fn validate_input(args: &Vec<String>) -> Result<(Command,&String), ValidationError>{
    if args.len() != 3 {
        return Err(ValidationError::InvalidArgumentCount { expected: 3, actual: args.len() });
    }

    let command =
        match Command::from_str(&args[1]) {
            Ok(command) => command,
            Err(error) => return Err(error)
        };

    /*
      Nice that pattern matching is a bit more robust than it is in Java.  Closer to what you
      can do in F#.
     */
    let filename =
        match fs::metadata(&args[2]) {
            Ok(metadata) if metadata.is_file() => &args[2],
            _ => return Err(ValidationError::InvalidFilename {
                provided_filename: args[2].to_string()
            })
        };

    Ok((command, filename))
}

fn execute_command(command: &Command, filename: &String) {
    match command {
        Command::Tokenize => tokenize(filename)
    }
}

fn handle_error(error: ValidationError) {
    eprintln!("{}", error);

    std::process::exit(
        match error {
            ValidationError::InvalidArgumentCount { .. } => exitcode::USAGE,
            ValidationError::InvalidCommand { .. } => exitcode::USAGE,
            ValidationError::InvalidFilename { .. } => exitcode::IOERR
        }
    )
}

fn main() {
    match validate_input(&env::args().collect()) {
        Err(error) => handle_error(error),
        Ok((command, filename)) => execute_command(&command, filename)
    }
}

fn tokenize(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let tokens = scanner.scan_tokens();

    for el in tokens {
        println!("{}", el)
    }

    if scanner.has_error() {
        std::process::exit(exitcode::DATAERR);
    }
}