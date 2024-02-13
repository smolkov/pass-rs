use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// subfolder
    subfolder: Option<PathBuf>,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
