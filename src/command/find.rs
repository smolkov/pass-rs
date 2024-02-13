use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Pass names  
    pass_name: String,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
