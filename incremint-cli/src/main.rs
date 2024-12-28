use chrono::{Datelike, Local};
use clap::Parser;
use incremint::{digit::Digits, increment::Incremint};

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    #[clap(default_value_t = Self::this_year())]
    prev: usize,
    #[arg(short, long)]
    #[clap(default_value_t = Self::next_year())]
    next: usize,
}
impl Cli {
    pub fn this_year() -> usize {
        Local::now().year() as usize
    }
    pub fn next_year() -> usize {
        Self::this_year() + 1
    }
}

fn main() {
    let cli = Cli::parse();
    let (prev, next) = (Digits::from(cli.prev), Digits::from(cli.next));
    println!("{}", Incremint { prev, next });
}
