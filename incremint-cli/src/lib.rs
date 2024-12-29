use chrono::{Datelike, Local};
use clap::Parser;
use incremint::increment::Incremint;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Parser)]
pub struct Cli {
    /// previous value of the increment
    #[arg(short, long)]
    #[clap(default_value_t = Self::this_year())]
    prev: usize,

    /// next value of the increment
    #[arg(short, long)]
    #[clap(default_value_t = Self::next_year())]
    next: usize,

    /// whitespace scale
    #[arg(short, long)]
    #[clap(default_value_t = 1)]
    scale: usize,
}
impl Cli {
    pub fn run() -> anyhow::Result<()> {
        let cli = Self::parse();
        let (prev, next) = (cli.prev.into(), cli.next.into());
        println!("{}", Incremint::new(prev, next).writer(cli.scale));
        Ok(())
    }
    pub fn this_year() -> usize {
        Local::now().year() as usize
    }
    pub fn next_year() -> usize {
        Self::this_year() + 1000
    }
}
