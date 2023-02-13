use crate::parsers::expr::Expr;

use crate::parsers::term::parse_term;

use nom::{branch::alt, bytes::complete::tag, multi::fold_many0, sequence::pair, IResult};

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
