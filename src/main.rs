mod parser;

use parser::parse;

fn main() {
    let result = parse("4 + 5 - 3 * 2");
    println!("{result}");
}
