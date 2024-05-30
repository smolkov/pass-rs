use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::store::Store;

use console::{style, Term};
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// subfolder
    subfolder: Option<PathBuf>,
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        let term = Term::stdout();
        writeln!(&term,"{}",style("Password store:").bold().blink())?;
        let ws_dir = store.directory();
        for entry in WalkDir::new(ws_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let ws_path = path.strip_prefix(ws_dir)?;
            if !ws_path.to_str().unwrap().starts_with('.') {
                if path.is_dir() {
                    writeln!(&term, "├── {}", style(ws_path.display()).blue().bold())?;
                } else {
                    writeln!(&term, "├── {}", ws_path.display())?;
                }
            }
        }
        Ok(())
    }
}
