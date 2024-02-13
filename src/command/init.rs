use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// Initialize new password storage and use gpg-id for encryption.

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Storage path (optional)
    #[arg(short, long)]
    path: Option<PathBuf>,
    /// GPG id ( optional)
    gpg_id: Option<String>,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let path = self.path.clone().unwrap_or_else(|| match dirs::home_dir() {
            Some(home) => home.join(".pass_store"),
            None => PathBuf::from(".pass_store"),
        });
        if path.exists() {
            return Err(anyhow::anyhow!(
                "initialize password storage in `{}` impossible - already exist",
                path.display()
            ));
        }
        let gpg_id = self.gpg_id.clone().unwrap_or("nokey".to_owned());
        std::fs::create_dir_all(&path)?;
        std::fs::write(path.join(".gpg-id"), gpg_id.trim())?;
        Ok(())
    }
}