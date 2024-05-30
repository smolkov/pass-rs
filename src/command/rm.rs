use std::path::PathBuf;
use std::fs;
use std::io::Write;

use anyhow::Result;
use clap::Parser;
use console::{Term,style};

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
    pub fn run(&self, store: &Store) -> Result<()> {
        let path = store.password(self.pass_name.as_path()); 
        let term = Term::stdout(); 
        if path.exists() {
            write!(&term,"Are you sure you would like to delete {}  {} ",style(self.pass_name.display()).cyan().bold(),style("[y/N]").red().bold())?;
            if term.read_line()?.starts_with('y') {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
}
