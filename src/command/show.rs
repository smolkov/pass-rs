use anyhow::Result;
use clap::Parser;

use crate::store::Store;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Put on the clipboard, it will be cleared in 45 seconds.
    #[arg(short, long, default_value_t = false)]
    clip: bool,
    /// password name
    pass_name: Option<String>,
}

impl Cli {
    pub fn run(&self, _store: &Store) -> Result<()> {
        Ok(())
    }
}
