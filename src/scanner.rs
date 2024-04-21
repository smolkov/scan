use anyhow::Result;
use std::ops::Range;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::timeout;

use crate::PortRange;

pub struct PortInfo {
    pub port: u16,
    pub open: bool,
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
        Scanner{
            rx
        // ports
        }
    }
    pub async fn recv(&mut self )  -> Option<PortInfo>{
        self.rx.recv().await
    }
    async fn scan_port(sender: Sender<PortInfo>, addr: String, port: u16) -> anyhow::Result<()> {
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
