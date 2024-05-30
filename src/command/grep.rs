use anyhow::Result;
use clap::Parser;

use crate::store::Store;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Search string
    search_string: String,
}

impl Cli {
    pub fn run(&self, _store: &Store) -> Result<()> {
        Ok(())
    }
}
