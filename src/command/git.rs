use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {}

impl Cli {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
