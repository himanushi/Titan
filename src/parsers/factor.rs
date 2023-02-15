use super::number::parse_number;
use crate::parsers::expr::Expr;
use crate::parsers::expression::parse_expression;

use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::map,
    sequence::delimited,
    IResult,
};

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            delimited(
                space0,
                delimited(char('('), parse_expression, char(')')),
                space0,
            ),
            |expr| Expr::Paren(Box::new(expr)),
        ),
        delimited(space0, parse_number, space0),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::expr::Expr::*;

    #[test]
    fn test_parse_factor_with_number() {
        let input = "123";
        let expected_output = Ok(("", Num(123.0)));
        let output = parse_factor(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_factor_with_negative_number() {
        let input = "-123";
        let expected_output = Ok(("", Num(-123.0)));
        let output = parse_factor(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_factor_with_parentheses() {
        let input = "(1 + 2)";
        let expected_output = Ok((
            "",
            Paren(Box::new(Add(Box::new(Num(1.0)), Box::new(Num(2.0))))),
        ));
        let output = parse_factor(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_factor_paren_mult() {
        let input = "(1 + 2) * 10";
        let expected_output = Ok((
            "",
            Expr::Mul(
                Box::new(Expr::Paren(Box::new(Expr::Add(
                    Box::new(Expr::Num(1.0)),
                    Box::new(Expr::Num(2.0)),
                )))),
                Box::new(Expr::Num(10.0)),
            ),
        ));
        let output = parse_factor(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_factor_mult_paren() {
        let input = "10 * (1 + 2)";
        let expected_output = Ok((
            "",
            Expr::Mul(
                Box::new(Expr::Num(10.0)),
                Box::new(Expr::Paren(Box::new(Expr::Add(
                    Box::new(Expr::Num(1.0)),
                    Box::new(Expr::Num(2.0)),
                )))),
            ),
        ));
        let output = parse_factor(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_factor_paren_mult_paren_mult() {
        let input = "(1 + 2) * (2 * 3)";
        let expected_output = Ok((
            "",
            Expr::Mul(
                Box::new(Expr::Paren(Box::new(Expr::Add(
                    Box::new(Expr::Num(1.0)),
                    Box::new(Expr::Num(2.0)),
                )))),
                Box::new(Expr::Paren(Box::new(Expr::Mul(
                    Box::new(Expr::Num(2.0)),
                    Box::new(Expr::Num(3.0)),
                )))),
            ),
        ));
        let output = parse_factor(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_factor_paren_mult_paren_div() {
        let input = "(1 + 2) * ((2 * 5) / 2)";
        let expected_output = Ok((
            "",
            Expr::Mul(
                Box::new(Expr::Paren(Box::new(Expr::Add(
                    Box::new(Expr::Num(1.0)),
                    Box::new(Expr::Num(2.0)),
                )))),
                Box::new(Expr::Paren(Box::new(Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Num(2.0)),
                        Box::new(Expr::Num(5.0)),
                    )),
                    Box::new(Expr::Num(2.0)),
                )))),
            ),
        ));
        let output = parse_factor(input);
        assert_eq!(output, expected_output);
    }
}
