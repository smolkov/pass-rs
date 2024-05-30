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

        Ok(())
    }
}
