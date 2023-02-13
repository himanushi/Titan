mod parsers;

fn main() {
    let input = "1 + 2 * 3 - 4 / 2";

    // パーサーの実行
    let (rest, result) = parsers::expression::parse_expression(input).unwrap();

    // パース結果の表示
    println!("Parsed successfully: {:?}", result);
    println!("Remaining input: {:?}", rest);
}
