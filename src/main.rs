mod tokens;
mod math;
mod parser;

use parser::parse_input;

fn main() {
    parse_input("1 + 5 * 4");
}
