use std::{path::PathBuf, process};
use std::fs;

use anyhow::Result;
use clap::Parser;

use crate::store::Store;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Put on the clipboard, it will be cleared in 45 seconds.
    #[arg(short, long, default_value_t = false)]
    clip: bool,
    /// password name
    pass_name: PathBuf,
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        let path = store.path().join(self.pass_name.as_path());
        if !path.exists() {
            eprintln!("password {} is not exist in store {}",self.pass_name.display(),store.path().display());
            process::exit(1);
        }

        let key = store.private_key()?;
        println!("{}",key.decrypt(fs::read_to_string(path)?.as_str())?);
        Ok(())
    }
}
