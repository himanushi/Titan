mod calc {
    pub mod calc;
}

use calc::calc::calculate;

fn main() {
    match calculate("1 + 2 * 3 - 4 / 2") {
        Ok(result) => println!("Result: {}", result),
        Err(msg) => eprintln!("Error: {}", msg),
    }
}
