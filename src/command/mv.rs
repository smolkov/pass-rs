use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Force insert new password
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Old path
    src: PathBuf,
    /// Moved to new path
    dest: PathBuf,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
