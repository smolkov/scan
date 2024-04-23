use clap::Parser;
use crate::cmd::Cmd;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Cmd,
    
}
