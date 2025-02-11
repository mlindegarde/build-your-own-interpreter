use crate::lexing::tokenizing::{Token, TokenData, TokenType};
use crate::parsing::parsing::Expression;

#[test]
fn should_generate_expected_output() {
    let minus_token = Token::new(1, TokenType::Minus, TokenData::Reserved { lexeme: "-" });
    let star_token = Token::new(1, TokenType::Star, TokenData::Reserved { lexeme: "*" });

    let expression = Expression::Binary {
        left: Box::from(Expression::Unary {
            operator: &minus_token,
            right: Box::from(Expression::StringLiteral { value: "123" })
        }),
        operator: &star_token,
        right: Box::from(Expression::Grouping {
            expression: Box::from(Expression::StringLiteral { value: "45.67" })
        })
    };

    let output = format!("{}", expression);
    assert_eq!(output, "(* (- 123) (group 45.67))");
}