use super::number::parse_number;
use super::variable::parse_variable;
use crate::parsers::expr::Expr;

use nom::{
    branch::alt, character::complete::space0, combinator::map, sequence::delimited, IResult,
};

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(space0, parse_number, space0),
        map(parse_variable, |var| Expr::Var(var)),
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
}
