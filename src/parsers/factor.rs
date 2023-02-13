use crate::parser::Expr;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    sequence::delimited,
    IResult,
};

pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(char('('), crate::parser::parse_expression, char(')')),
        crate::parser::parse_number,
    ))(input)
}
