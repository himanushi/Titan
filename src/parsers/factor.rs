use crate::parsers::expr::Expr;
use crate::parsers::number::parse_number;
use crate::parsers::expression::parse_expression;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(char('('), parse_expression, char(')')),
        parse_number,
    ))(input)
}
