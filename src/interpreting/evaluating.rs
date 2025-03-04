/*
use std::fs;
use exitcode::ExitCode;
use crate::lexing::scanning::Scanner;
use crate::parsing::parsing::Parser;

pub fn evaluate_ast(filename: &str.) -> exitcode {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}:  Defaulting to an empty string", filename);
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    let scannerResult = &scanner.scan_tokens();

    exitcode::OK;
}
*/