use chrono::{Datelike, Local};
use clap::Parser;
use incremint::{digit::IntoDigits, increment::Incremint};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Parser)]
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (prev, next) = (cli.prev.into_digits(), cli.next.into_digits());
    println!("{}", Incremint { prev, next });
    Ok(())
}
