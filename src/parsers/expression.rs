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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        assert_eq!(
            parse_expression("1+2*3-4/2"),
            Ok((
                "",
                Expr::Sub(
                    Box::new(Expr::Add(
                        Box::new(Expr::Num(1.0)),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Num(2.0)),
                            Box::new(Expr::Num(3.0))
                        ))
                    )),
                    Box::new(Expr::Div(
                        Box::new(Expr::Num(4.0)),
                        Box::new(Expr::Num(2.0))
                    ))
                )
            ))
        );

        assert_eq!(
            parse_expression("10-5-3"),
            Ok((
                "",
                Expr::Sub(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Num(10.0)),
                        Box::new(Expr::Num(5.0))
                    )),
                    Box::new(Expr::Num(3.0))
                )
            ))
        );

        assert_eq!(parse_expression("1"), Ok(("", Expr::Num(1.0))));

        assert_eq!(
            parse_expression("2*3/6"),
            Ok((
                "",
                Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Num(2.0)),
                        Box::new(Expr::Num(3.0))
                    )),
                    Box::new(Expr::Num(6.0))
                )
            ))
        );
    }
}
