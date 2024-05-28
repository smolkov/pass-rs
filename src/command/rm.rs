use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::store::Store;

#[derive(Debug, Parser)]
pub struct Cli {
    /// remove recursive
    #[arg(short, long, default_value_t = false)]
    recursive: bool,
    /// Password path to remove
    pass_name: PathBuf,
}

impl Cli {
    pub fn run(&self, _store: &Store) -> Result<()> {
        Ok(())
    }
}
