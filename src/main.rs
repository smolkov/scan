use clap::Parser;
use scann::cli::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    args.command.run()?;
    Ok(())
}
