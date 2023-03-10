extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// パーサーを定義する
#[derive(Parser)]
#[grammar = "parser.pest"]
struct ExpressionParser;

// メイン関数を定義する
fn main() {
    // 式を取得する
    let expression = std::env::args().nth(1).unwrap();

    // 式をパースする
    let pairs =
        ExpressionParser::parse(Rule::parser, &expression).unwrap_or_else(|e| panic!("{}", e));

    // パースした結果を表示する
    for pair in pairs {
        println!("{:?}", pair);
    }
}
