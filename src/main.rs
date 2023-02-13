mod parsers;
use parsers::parser::parse_expression;

fn main() {
    let input = "1 + 2 * 3 - 4 / 2";

    // パーサーの実行
    let (rest, result) = parse_expression(&input).unwrap();

    // パース結果の表示
    println!("Parsed successfully: {:?}", result);
    println!("Remaining input: {:?}", rest);
}
