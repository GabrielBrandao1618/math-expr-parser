use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub expression: String,
}
