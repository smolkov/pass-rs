use std::io::Write;
use std::path::PathBuf;
use std::fs;
use anyhow::Result;
use clap::Parser;
use console::Term;
use console::style;

use crate::store::Store;

#[derive(Debug,Parser)]
pub struct Cli {
    /// Edit password 
    path: PathBuf,
}

impl Cli {
    /// Run `pass edit` command
    pub fn run(&self,store:&Store) -> Result<()> {
        let path = store.password(self.path.as_path());
        if path.is_file() {
            let name = path.file_name().unwrap();
            let key = store.private_key().map_err(|_|anyhow::anyhow!("Store get private key error"))?;
            let term = Term::stdout();
            // let pass = key.decrypt(&fs::read_to_string(&path)?)?;
            term.show_cursor()?;
            write!(&term, "Enter new password for {}: ",style(name.to_str().unwrap()).red().bold())?;
            // writeln!(&term, "Edit password : ")?;
            let pass = term.read_line()?;
            // writeln!(&term,"\npass:{}",&pass)?;
            let pass = key.encrypt(pass.as_bytes())?;
            fs::write(path, pass.trim())?;
        }
        Ok(())
    }
}
