use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(recognize(pair(opt(tag("-")), digit1)), |s: &str| {
        Expr::Num(s.parse().unwrap())
    })(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(char('('), parse_expression, char(')')),
        parse_number,
    ))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
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
