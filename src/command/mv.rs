use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;


use crate::store::Store;

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
    pub fn run(&self,store:&Store) -> Result<()> {
        let src = store.password(self.src.as_path()); 
        let dest = store.password(self.dest.as_path()); 
        let parent = dest.parent().ok_or(anyhow::anyhow!("destination file path parent not found"))?;
        if !parent.is_dir() {
            fs::create_dir_all(parent)?;
        }
        if src.exists() {
            fs::rename(src,dest)?;
        }
        Ok(())
    }
}
