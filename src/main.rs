use nom::{
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = std::env::args().nth(1).unwrap();

    // パーサーの実行
    let (rest, result) = parse_expression(&input).unwrap();

    // パース結果の表示
    println!("Parsed successfully: {:?}", result);
    println!("Remaining input: {:?}", rest);
}

// 数値リテラルをパースするパーサー
fn parse_number(input: &str) -> IResult<&str, f64> {
    map(
        recognize(tuple((
            opt(alt((char('+'), char('-')))),
            alt((digit1, preceded(char('.'), digit1))),
            opt(preceded(char('e'), alt((char('+'), char('-'))))),
            opt(digit1),
        ))),
        |s: &str| s.parse().unwrap(),
    )(input)
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
    loop {
        if let Ok((rest, op)) = alt((tag("+"), tag("-")))(rest) {
            let (rest, _) = multispace0(rest)?;
            let (rest, rhs) = parse_term(rest)?;
            lhs = match op {
                "+" => Expr::Add(Box::new(lhs), Box::new(rhs)),
                "-" => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            };
        } else {
            break;
        }
    }
    Ok((rest, lhs))
}

// 因子をパースするパーサー
fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        map(parse_number, Expr::Number),
        delimited(
            multispace0,
            delimited(tag("("), parse_expression, delimited(tag(")"), multispace0)),
            multispace0,
        ),
    ))(input)
}

// 式を表すデータ型
#[derive(Debug)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
