mod parsers

use crate::parsers::expr::Expr;
use crate::parsers::expression::parse_expression;
use nom::Finish;

pub fn calculate(input: &str) -> Result<f64, String> {
    let result = parse_expression(input).finish();

    match result {
        Ok((_, ast)) => Ok(eval(&ast)),
        Err(e) => Err(format!("Parse error: {}", e.to_string())),
    }
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(num) => *num,
        Expr::Add(left, right) => eval(left) + eval(right),
        Expr::Sub(left, right) => eval(left) - eval(right),
        Expr::Mul(left, right) => eval(left) * eval(right),
        Expr::Div(left, right) => eval(left) / eval(right),
    }
}
