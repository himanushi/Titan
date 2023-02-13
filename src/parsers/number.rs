use crate::parsers::expr::Expr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, opt, recognize},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

pub fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(recognize(pair(opt(tag("-")), digit1)), |s: &str| {
        Expr::Num(s.parse().unwrap())
    })(input)
}
