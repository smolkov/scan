use anyhow::Result;
use clap::Parser;

#[derive(Debug,Parser)]
pub struct Tui {

}



impl Tui {
	pub async fn run(&self) -> Result<()>{
		Ok(())
	}
}