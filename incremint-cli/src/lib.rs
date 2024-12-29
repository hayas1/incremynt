use std::io::Write;

use chrono::{Datelike, Local};
use clap::{Parser, ValueEnum};
use incremint::{increment::Incremint, interface::Application, space::Width};

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
impl From<Cli> for Application<Incremint> {
    fn from(cli: Cli) -> Self {
        Application::<Incremint> {
            d: (cli.prev, cli.next).into(),
            space: cli.space.into(),
            scale: cli.scale,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ValueEnum)]
pub enum SpaceWidth {
    #[default]
    Half,
    Full,
}
impl From<SpaceWidth> for Width {
    fn from(width: SpaceWidth) -> Self {
        match width {
            SpaceWidth::Half => Width::Half,
            SpaceWidth::Full => Width::Full,
        }
    }
}

impl Cli {
    pub fn exec() -> anyhow::Result<()> {
        let cli = Self::parse();
        cli.run(&mut std::io::stdout().lock())
    }
    pub fn run<W: Write>(self, w: &mut W) -> anyhow::Result<()> {
        Ok(Application::<Incremint>::from(self).run(w)?)
    }
    pub fn this_year() -> usize {
        Local::now().year() as usize
    }
    pub fn next_year() -> usize {
        Self::this_year() + 1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_behavior() {
        let cli = Cli::try_parse_from(["incremint", "-p", "2024", "-n", "3024"]).unwrap();

        assert_eq!(
            cli,
            Cli {
                prev: 2024,
                next: 3024,
                space: SpaceWidth::Half,
                scale: 1
            }
        );

        let mut buffer = Vec::new();
        cli.run(&mut buffer).unwrap();
        assert_eq!(
            String::from_utf8_lossy(&buffer),
            vec![
                "┏━┛┃            ",
                "┗━┓┃┏━━┓┏━━┓┏┓┏┓",
                "┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃",
                "┗━━┛┃┃┃┃┏━┛┃┃┗┛┃",
                "┏━━┓┃┃┃┃┃┏━┛┗━┓┃",
                "┗━┓┃┃┗┛┃┃┗━┓  ┃┃",
                "┏━┛┃┗━━┛┗━━┛  ┗┛",
                "┃┏━┛            ",
                "",
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_cli_empty_arg() {
        let cli = Cli::try_parse_from(["incremint"]).unwrap();
        assert!(cli.run(&mut std::io::empty()).is_ok());
    }
}
