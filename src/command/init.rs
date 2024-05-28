use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use crate::store::Store;

/// Initialize new password storage and use gpg-id for encryption.

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// GPG id ( optional)
    gpg_id: String,
}

impl Cli {
    pub fn run(&self,_store:&Store) -> Result<()> {
        // let path = self.path.clone().unwrap_or_else(|| match dirs::home_dir() {
            // Some(home) => home.join(".pass_store"),
            // None => PathBuf::from(".pass_store"),
        // });
        // if path.exists() {
            // return Err(anyhow::anyhow!(
                // "initialize password storage in `{}` impossible - already exist",
                // path.display()
            // ));
        // }
        // std::fs::create_dir_all(&path)?;
        // std::fs::write(path.join(".gpg-id"), self.gpg_id.trim())?;
        Ok(())
    }
}