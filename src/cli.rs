use std::str::FromStr;

use clap::Parser;
use crate::PortRange;
use anyhow::Result;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub address: String,

    #[arg(short, long)]
    pub range: Option<String>,
}



impl Args {
    pub fn port_range(&self) ->  Result<PortRange> {
        let range = if let Some(range)  = self.range.as_ref()  {
            PortRange::from_str(range.as_str())?
        }else {
            PortRange::new(0, 65535)
        };
        Ok(range)
    }
}