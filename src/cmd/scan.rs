use anyhow::Result;
use clap::Parser;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::timeout;

use crate::port::{PortInfo,PortRange};

#[derive(Parser, Debug)]
pub struct Scan {
    /// ip address
    #[arg(short, long)]
    pub address: String,
    /// port range ( default 0..65535)
    #[arg(short, long, default_value = "0..65535")]
    pub range: String,
}

impl Scan {
    pub async fn run(&self) -> Result<()> {
        let mut scanner = Scanner::scan(&self.address, self.port_range()?);
        tokio::spawn(async move {
            while let Some(msg) = scanner.recv().await {
                if msg.open {
                    println!("port:{} {}", msg.port, msg.open);
                }
            }
        }).await?;
        Ok(())
    }
    pub fn port_range(&self) -> Result<PortRange> {
        Ok(PortRange::from_str(self.range.as_str())?)
    }
}

pub struct Scanner {
    rx: Receiver<PortInfo>,
}

impl Scanner {
    pub fn scan(address: &str, range: PortRange) -> Scanner {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1000);
        let address = address.to_owned();
        tokio::spawn(async move {
            for port in range {
                tokio::spawn(Scanner::scan_port(tx.clone(), address.clone(), port));
            }
        });
        Scanner { rx }
    }
    pub async fn recv(&mut self) -> Option<PortInfo> {
        let info = self.rx.recv().await;
        info
    }
    async fn scan_port(sender: Sender<PortInfo>, addr: String, port: u32) -> anyhow::Result<()> {
        let duration = Duration::from_millis(1800);
        if let Ok(Ok(_)) = timeout(
            duration,
            tokio::net::TcpStream::connect(format!("{}:{}", addr, port)),
        )
        .await
        {
            sender
                .send(PortInfo {
                    port: port,
                    open: true,
                })
                .await?;
        } else {
            sender
                .send(PortInfo {
                    port: port,
                    open: false,
                })
                .await?;
        }
        Ok(())
    }
}
