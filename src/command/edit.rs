use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::store::Store;

#[derive(Debug,Parser)]
pub struct Cli {
    /// Edit password 
    #[arg(short, long)]
    path: PathBuf,
}

impl Cli {
    /// Run `pass edit` command
    pub fn run(&self,store:&Store) -> Result<()> {
        Ok(())
    }
}
