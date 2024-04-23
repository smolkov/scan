use anyhow::Result;
use clap::Parser;


#[derive(Debug,Parser)]
pub struct Watch{
	  /// watch tcp connections
	  #[arg(long, default_value_t=true)]
	  tcp :bool,
	  /// watch udp connections 
	  #[arg(long, default_value_t=false)]
	  udp: bool
}


impl Watch {
	pub fn run(&self ) -> Result<()> {
		
		Ok(())		
	}
}