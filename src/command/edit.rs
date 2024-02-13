use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Debug,Parser)]
pub struct Cli {
    /// Storage path (optional)
    #[arg(short, long)]
    path: Option<PathBuf>,
    /// GPG id ( optional)
    gpg_id: Option<String>,
}

impl Cli {
    /// Run `pass edit` command
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
