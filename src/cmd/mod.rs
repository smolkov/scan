use anyhow::Result;
use clap::Subcommand;

pub mod scan;
pub mod watch;
pub mod tui;

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Scan(scan::Scan),
    Watch(watch::Watch),
    Tui(tui::Tui),
}

impl Cmd {
    pub async fn run(&self) -> Result<()> {
        match self {
            Cmd::Scan(cmd) => cmd.run().await?,
            Cmd::Watch(cmd) => cmd.run().await?,
            Cmd::Tui(cmd) => cmd.run().await?,
        }
        Ok(())
    }
}
