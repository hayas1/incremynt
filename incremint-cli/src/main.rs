use clap::Parser;
use incremint::{digit::Digits, increment::Incremint};

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    prev: usize,
    #[arg(short, long)]
    next: usize,
}

fn main() {
    let cli = Cli::parse();
    let (prev, next) = (Digits::from(cli.prev), Digits::from(cli.next));
    println!("{}", Incremint { prev, next });
}
