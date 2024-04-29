use std::any;

use anyhow::Result;
use etherparse::LaxPacketHeaders;
use etherparse::{NetHeaders, TransportHeader};

pub struct PacketAddr {
    pub ip: String,
    pub port: u16,
}

pub struct PacketHeader {
    pub source: PacketAddr,
    pub dest: PacketAddr,
}

impl PacketHeader {
    pub fn from_pcap_packet(packet: &pcap::Packet) -> Result<PacketHeader> {
        let header = get_packet_headers(packet)?;
        let (src_ip, dest_ip) = header_get_ip(&header);
        let (src_port, dest_port) = header_get_ports(&header);
        let source = PacketAddr {
            ip: src_ip,
            port: src_port,
        };
        let dest = PacketAddr {
            ip: dest_ip,
            port: dest_port,
        };
        Ok(PacketHeader { source, dest })
    }
}

fn header_get_ip(header: &LaxPacketHeaders) -> (String, String) {
    if let Some(net) = header.net.as_ref() {
        match net {
            NetHeaders::Ipv4(header, _) => (
                format!(
                    "{}.{}.{}.{}",
                    header.source[0], header.source[1], header.source[2], header.source[3]
                ),
                format!(
                    "{}.{}.{}.{}",
                    header.destination[0],
                    header.destination[1],
                    header.destination[2],
                    header.destination[3]
                ),
            ),
            NetHeaders::Ipv6(header, _) => (
                format!(
                    "{}.{}.{}.{}",
                    header.source[0], header.source[1], header.source[2], header.source[3]
                ),
                format!(
                    "{}.{}.{}.{}",
                    header.destination[0],
                    header.destination[1],
                    header.destination[2],
                    header.destination[3]
                ),
            ),
        }
    } else if let Some(link) = header.link.as_ref() {
        (
            format!(
                "{}.{}.{}.{}",
                link.source[0], link.source[1], link.source[2], link.source[3]
            ),
            format!(
                "{}.{}.{}.{}",
                link.destination[0], link.destination[1], link.destination[2], link.destination[3]
            ),
        )
    } else {
        ("unknown".to_owned(), "unknown".to_owned())
    }
}

fn header_get_ports(header: &LaxPacketHeaders) -> (u16, u16) {
    if let Some(transport) = header.transport.as_ref() {
        match transport {
            TransportHeader::Udp(header) => (header.source_port, header.destination_port),
            TransportHeader::Tcp(header) => (header.source_port, header.destination_port),
            TransportHeader::Icmpv4(_) => (0, 0),
            TransportHeader::Icmpv6(_) => (0, 0),
        }
    } else {
        (0, 0)
    }
}

fn get_packet_headers<'a>(packet: &'a pcap::Packet) -> Result<LaxPacketHeaders<'a>> {
    let header = LaxPacketHeaders::from_ethernet(packet)?;
    Ok(header)
}

fn header_get_source_ip(header: &LaxPacketHeaders) -> String {
    if let Some(net) = header.net.as_ref() {
        match net {
            NetHeaders::Ipv4(header, _) => {
                format!(
                    "{}.{}.{}.{}",
                    header.source[0], header.source[1], header.source[2], header.source[3]
                )
            }
            NetHeaders::Ipv6(header, _) => {
                format!(
                    "{}.{}.{}.{}",
                    header.source[0], header.source[1], header.source[2], header.source[3]
                )
            }
        }
    } else if let Some(link) = header.link.as_ref() {
        format!(
            "{}.{}.{}.{}",
            link.source[0], link.source[1], link.source[2], link.source[3]
        )
    } else {
        "unknown".to_owned()
    }
}

fn header_get_dest_ip(header: &LaxPacketHeaders) -> String {
    if let Some(net) = header.net.as_ref() {
        match net {
            NetHeaders::Ipv4(header, _) => {
                format!(
                    "{}.{}.{}.{}",
                    header.destination[0],
                    header.destination[1],
                    header.destination[2],
                    header.destination[3]
                )
            }
            NetHeaders::Ipv6(header, _) => {
                format!(
                    "{}.{}.{}.{}",
                    header.destination[0],
                    header.destination[1],
                    header.destination[2],
                    header.destination[3]
                )
            }
        }
    } else if let Some(link) = header.link.as_ref() {
        format!(
            "{}.{}.{}.{}",
            link.destination[0], link.destination[1], link.destination[2], link.destination[3]
        )
    } else {
        "unknown".to_owned()
    }
}
