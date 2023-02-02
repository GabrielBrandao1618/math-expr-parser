mod parser;

use parser::parse;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pub expression: String,
}

fn main() {
    let args = Cli::parse();
    let result = parse(&args.expression);
    println!("{result}");
}
