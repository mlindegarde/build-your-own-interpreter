use regex::Regex;

/// CodeCrafters wants the output of the TokenType enum to be in UPPER_SNAKE_CASE; however,
/// Rust seems to want enums to use PascalCase.  To fix this, this simple method will convert
/// Pascal to upper snake case, ex:
/// ```
/// ExampleValue -> EXAMPLE_VALUE
/// ```
pub fn pascal_to_upper_case_snake(input: &String) -> String {
    let regex =  Regex::new("[A-Z][a-z]+");

    match regex {
        Ok(regex) => {
            regex.find_iter(input)
                .map(|m| m.as_str().to_uppercase())
                .collect::<Vec<String>>()
                .join("_")
        }
        Err(_) => {
            eprintln!("Somehow failed to create the regular expression for {:?}", regex);
            String::from(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_pascal_to_upper_snake() {
        let input = String::from("InputString");
        let expected_output = "INPUT_STRING";
        let output = pascal_to_upper_case_snake(&input);

        assert_eq!(output, expected_output);
    }
}
