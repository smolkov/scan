use anyhow::Result;
use std::ops::Range;
use tokio::sync::mpsc::{Receiver, Sender};

struct PortMessage {
    port: u16,
    open: bool,
}

pub struct Scanner {
    ports: Range<u16>,
    rx: Receiver<PortMessage>,
}

impl Scanner {
    // pub fn new() -> Scanner {
        // Scanner{
        // ports
        // }
    // }

    pub async fn scan(addr: &str, ports: Range<u16>) -> Result<()> {
        // tokio::spawn(async move { for port in self.ports {
			
		// } });

        Ok(())
    }
}
