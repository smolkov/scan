use anyhow::Result;
use clap::Subcommand;

pub mod scan;
pub mod watch;

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Scan(scan::Scan),
    Watch(watch::Watch),
}

impl Cmd {
    pub fn run(&self) -> Result<()> {
        match self {
            Cmd::Scan(scan) => scan.run()?,
            Cmd::Watch(watch) => watch.run()?,
        }
        Ok(())
    }
}
