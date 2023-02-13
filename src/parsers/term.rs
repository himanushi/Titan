use crate::parsers::expr::Expr;
use crate::parsers::factor::parse_factor;

use nom::{branch::alt, bytes::complete::tag, multi::fold_many0, sequence::pair, IResult};

pub fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (rest, lhs) = parse_factor(input)?;
    fold_many0(
        pair(alt((tag("*"), tag("/"))), parse_factor),
        lhs,
        |acc, (op, rhs)| match op {
            "*" => Expr::Mul(Box::new(acc), Box::new(rhs)),
            "/" => Expr::Div(Box::new(acc), Box::new(rhs)),
            _ => unreachable!(),
        },
    )(rest)
}
