mod math;
mod parser;
mod tokens;
mod resolve_expression;
mod cli;

use clap::Parser;
use cli::Cli;

use resolve_expression::resolve_expression;

fn main() {
    let args = Cli::parse();
    let result = resolve_expression(&args.expression);
    println!("{}", result);
}
