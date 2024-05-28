use anyhow::Result;
use clap::Parser;

use crate::store::Store;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Pass names  
    pass_name: String,
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        Ok(())
    }
}
