use crate::parsers::expr::Expr;
use crate::parsers::expression::parse_expression;
use crate::parsers::number::parse_number;

use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::map,
    sequence::delimited,
    IResult,
};

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(
            space0,
            map(delimited(char('('), parse_expression, char(')')), |expr| {
                Expr::Paren(Box::new(expr))
            }),
            space0,
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
}
