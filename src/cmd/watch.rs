use anyhow::Result;
use clap::Parser;
use pcap::{Device,Packet};

use crate::packet::PacketHeader;


#[derive(Debug, Parser)]
pub struct Watch {
    /// watch tcp connections
    #[arg(long, default_value_t = true)]
    tcp: bool,
    /// watch udp connections
    #[arg(long, default_value_t = false)]
    udp: bool,
}

impl Watch {
    pub async fn run(&self) -> Result<()> {
            let device = Device::lookup()
            .expect("device lookup failed")
            .expect("no device available");
            println!("Using device {}", device.name);

        // Setup Capture
        let mut cap = pcap::Capture::from_device(device)
            .unwrap()
            .immediate_mode(true)
            .open()
            .unwrap();

        while let Ok(packet) = cap.next_packet() {
                // println!("received packet! {:?}", packet);
                match PacketHeader::from_pcap_packet(&packet) {
                    Err(e) => println!("error {}",e),
                    Ok(header) => {
                        println!("IP:{}:{}->{}:{}",header.source.ip,header.source.port,header.dest.ip,header.dest.port);
                    }
                }
        }
       
        Ok(())
    }
}

