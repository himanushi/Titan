extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// パーサーを定義する
#[derive(Parser)]
#[grammar = "expression.pest"]
struct ExpressionParser;

// メイン関数を定義する
fn main() {
    // 式を取得する
    let expression = std::env::args().nth(1).unwrap();

    // 式をパースする
    let pairs =
        ExpressionParser::parse(Rule::expression, &expression).unwrap_or_else(|e| panic!("{}", e));

    // パースした結果を表示する
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        println!("{}", "");

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!(),
            };
        }
    }
}
