use crate::parsers::expr::Expr;
use crate::parsers::factor::parse_factor;
use crate::parsers::number::parse_number;
use crate::parsers::term::parse_term;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

pub fn parse_expression(input: &str) -> IResult<&str, Expr> {
    let (rest, lhs) = parse_term(input)?;
    fold_many0(
        pair(alt((tag("+"), tag("-"))), parse_term),
        lhs,
        |acc, (op, rhs)| match op {
            "+" => Expr::Add(Box::new(acc), Box::new(rhs)),
            "-" => Expr::Sub(Box::new(acc), Box::new(rhs)),
            _ => unreachable!(),
        },
    )(rest)
}
