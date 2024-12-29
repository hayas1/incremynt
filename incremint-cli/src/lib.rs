use chrono::{Datelike, Local};
use clap::{Parser, ValueEnum};
use incremint::{increment::Incremint, space::Width};

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

    /// whitespace width
    #[arg(short, long)]
    #[clap(value_enum, default_value_t)]
    space: SpaceWidth,

    /// whitespace scale
    #[arg(long)]
    #[clap(default_value_t = 1)]
    scale: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ValueEnum)]
pub enum SpaceWidth {
    #[default]
    Half,
    Full,
}
impl Into<Width> for SpaceWidth {
    fn into(self) -> Width {
        match self {
            SpaceWidth::Half => Width::Half,
            SpaceWidth::Full => Width::Full,
        }
    }
}

impl Cli {
    pub fn run() -> anyhow::Result<()> {
        let cli = Self::parse();
        let (prev, next) = (cli.prev.into(), cli.next.into());
        println!(
            "{}",
            Incremint::new(prev, next).writer(cli.space.into(), cli.scale)
        );
        Ok(())
    }
    pub fn this_year() -> usize {
        Local::now().year() as usize
    }
    pub fn next_year() -> usize {
        Self::this_year() + 1000
    }
}
