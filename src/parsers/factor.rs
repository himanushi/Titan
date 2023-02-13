use crate::parsers::expr::Expr;
use crate::parsers::expression::parse_expression;
use crate::parsers::number::parse_number;

use nom::{
    branch::alt,
    character::complete::{char, space0},
    sequence::{delimited},
    IResult,
};

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(
            space0,
            delimited(char('('), parse_expression, char(')')),
            space0,
        ),
        parse_number,
    ))(input)
}
