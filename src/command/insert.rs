use std::path::PathBuf;
use std::io::Write;
use std::fs;

use anyhow::Result;
use clap::Parser;
use console::{style, Term};

use crate::store::Store;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Force insert new password
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Password address
    pass_name: PathBuf,
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        let path = store.password(self.pass_name.as_path());
        let _key = store.private_key()?;
        let term = Term::stdout();
        if path.exists() {
            write!(
                &term,
                "An entry already exists for {}. Overwrite it? {} ",
                style(self.pass_name.display()).cyan().bold(),
                style("[y/N]").red().bold()
            )?;
            let user_input = term.read_line()?;
            if !user_input.starts_with('y') {
                return Ok(());
            }
        }else {
            let parent =path.parent().unwrap();
            if !parent.is_dir()  {
                fs::create_dir_all(parent)?;
            }
        }
        let key = store.private_key().map_err(|_|anyhow::anyhow!("Store get private key error"))?;
        let name = path.file_name().unwrap();
        write!(&term, "Enter new password for {}: ",style(name.to_str().unwrap()).red().bold())?;
        let pass = term.read_line()?;
        let pass = key.encrypt(pass.as_bytes())?;
        fs::write(&path, pass.trim())?;
        store.git()?.add(path.strip_prefix(store.directory())?)?;
        Ok(())
    }
}
