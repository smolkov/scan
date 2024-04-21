use clap::Parser;

use scann::cli::Args;
use scann::scanner::{PortInfo, Scanner};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut scanner = Scanner::scan(&args.address, args.port_range()?);
    while let Some(msg) = scanner.recv().await {
        if msg.open {
            println!("port:{} {}", msg.port, msg.open);
        }
    }
    Ok(())
}
