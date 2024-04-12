use clap::Parser;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::timeout;

use scann::cli::Args;

struct PortMessage {
    port: u16,
    open: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (tx, mut rx) = tokio::sync::mpsc::channel(1000);
    tokio::spawn(async move {
        for port in 1..65000 {
            tokio::spawn(scan_port(tx.clone(), args.address.clone(), port));
        }
    });
    while let Some(msg) = rx.recv().await {
        if msg.open {
            println!("port:{} {}", msg.port, msg.open);
        }
    }
    Ok(())
}

async fn scan_port(sender: Sender<PortMessage>, addr: String, port: u16) -> anyhow::Result<()> {
    let duration = Duration::from_millis(1800);
    if let Ok(Ok(_)) = timeout(
        duration,
        tokio::net::TcpStream::connect(format!("{}:{}", addr, port)),
    )
    .await
    {
        sender
            .send(PortMessage {
                port: port,
                open: true,
            })
            .await?;
    } else {
        sender
            .send(PortMessage {
                port: port,
                open: false,
            })
            .await?;
    }
    Ok(())
}
