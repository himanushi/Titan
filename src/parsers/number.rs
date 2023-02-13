use crate::parsers::expr::Expr;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, digit1},
    combinator::{map, opt, recognize},
    sequence::{pair, tuple},
    IResult,
};

pub fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(
        recognize(tuple((
            opt(char('-')),
            digit1,
            opt(tuple((
                char('.'),
                take_while_m_n(1, 20, |c: char| c.is_ascii_digit()),
            ))),
        ))),
        |s: &str| match s.parse() {
            Ok(num) => Expr::Num(num),
            Err(_) => Expr::Float(s.parse().unwrap()),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_number_with_positive_integer() {
        let input = "123";
        let expected_output = Ok(("", Expr::Num(123.0)));
        let output = parse_number(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_number_with_negative_integer() {
        let input = "-123";
        let expected_output = Ok(("", Expr::Num(-123.0)));
        let output = parse_number(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_number_with_positive_float() {
        let input = "1.23";
        let expected_output = Ok(("", Expr::Num(1.23)));
        let output = parse_number(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_parse_number_with_negative_float() {
        let input = "-1.23";
        let expected_output = Ok(("", Expr::Num(-1.23)));
        let output = parse_number(input);
        assert_eq!(output, expected_output);
    }
}
