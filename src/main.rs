mod math;
mod parser;
mod tokens;
mod resolve_expression;

use resolve_expression::resolve_expression;

fn main() {
    let result = resolve_expression("1 + 2 + 3");
    println!("{}", result);
}
