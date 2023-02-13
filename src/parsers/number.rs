use crate::parsers::expr::Expr;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt, recognize},
    sequence::pair,
    IResult,
};

pub fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(recognize(pair(opt(tag("-")), digit1)), |s: &str| {
        Expr::Num(s.parse().unwrap())
    })(input)
}
