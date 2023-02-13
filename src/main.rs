use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated, tuple, recognize},
    IResult,
};

// 式のAST（抽象構文木）を表すデータ型
#[derive(Debug)]
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

// 数値をパースするパーサー
fn parse_number(input: &str) -> IResult<&str, f64> {
    map(
        recognize(tuple((
            opt(alt((char('+'), char('-')))),
            alt((digit1, preceded(char('.'), digit1))),
            opt(preceded(char('e'), opt(alt((char('+'), char('-')))))),
        ))),
        |s: &str| s.parse().unwrap(),
    )(input)
}

// 因子をパースするパーサー
fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        map(delimited(char('('), parse_expression, char(')')), |expr| expr),
        map(parse_number, Expr::Num),
    ))(input)
}

// 項をパースするパーサー
fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (rest, mut lhs) = parse_factor(input)?;
    let (rest, _) = multispace0(rest)?;
    loop {
        if let Ok((rest, op)) = alt((tag("*"), tag("/")))(rest) {
            let (rest, _) = multispace0(rest)?;
            let (rest, rhs) = parse_factor(rest)?;
            lhs = match op {
                "*" => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                "/" => Expr::Div(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            };
        } else {
            break;
        }
    }
    Ok((rest, lhs))
}

// 式をパースするパーサー
fn parse_expression(input: &str) -> IResult<&str, Expr> {
    let (rest, mut lhs) = parse_term(input)?;
    let (rest, _) = multispace0(rest)?;
    let (rest, result) = fold_many0(
        pair(alt((tag("+"), tag("-"))), parse_term),
        lhs,
        |acc, (op, rhs)| match op {
            "+" => Expr::Add(Box::new(acc), Box::new(rhs)),
            "-" => Expr::Sub(Box::new(acc), Box::new(rhs)),
            _ => unreachable!(),
        },
    )(rest)?;
    Ok((rest, result))
}

fn main() {
    let input = "1 + 2 * 3 - 4 / 2";

    // パーサーの実行
    let (rest, result) = parse_expression(&input).unwrap();

    // パース結果の表示
    println!("Parsed successfully: {:?}", result);
    println!("Remaining input: {:?}", rest);
}
