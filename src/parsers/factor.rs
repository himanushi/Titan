use crate::parsers::expr::Expr;
use crate::parsers::expression::parse_expression;
use crate::parsers::number::parse_number;

use nom::{
    branch::alt,
    character::complete::{char},
    sequence::{delimited},
    IResult,
};

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(char('('), parse_expression, char(')')),
        parse_number,
    ))(input)
}
