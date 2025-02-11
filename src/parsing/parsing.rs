use std::fs;
use exitcode::ExitCode;
use crate::lexing::scanning::Scanner;
use crate::lexing::tokenizing;

pub fn parse_file(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let tokens = Scanner::new(file_contents).scan_tokens().unwrap_or_else(|(tokens, errors)| {
        tokenizing::display_errors(&errors);
        tokenizing::display_tokens(&tokens);

        Vec::new()
    });

    if tokens.len() != 0 { exitcode::OK }
    else { exitcode::DATAERR }
}