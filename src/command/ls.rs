use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::store::Store;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// subfolder
    subfolder: Option<PathBuf>,
}

impl Cli {
    pub fn run(&self, _store: &Store) -> Result<()> {
        Ok(())
    }
}
