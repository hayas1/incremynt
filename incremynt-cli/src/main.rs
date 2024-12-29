use incremynt_cli::Cli;

fn main() -> anyhow::Result<()> {
    Ok(Cli::exec()?)
}
