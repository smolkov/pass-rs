use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use console::Term;
use console::style;

use crate::dirs;
use crate::generator::PasswordGenerator;
use crate::store::Store;


#[derive(Debug, Parser)]
pub struct Cli {
    /// Force insert new password
    #[arg(short, long, default_value_t = false)]
    no_symbols: bool,
    /// Optionally put on the clipboard, it will be cleared in 45 seconds.
    #[arg(short, long, default_value_t = false)]
    clip: bool,
    /// Force generate
    #[arg(short, long, default_value_t = false)]
    force: bool,
    /// Password name
    pass_name: PathBuf,
    /// Password length
    pass_len: Option<u8>,
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        let path = store.password(&self.pass_name);
        let key = store.private_key()?;
        let term = Term::stdout(); 
        if path.exists() && ! self.force {
            write!(&term,"An entry already exists for {}. Overwrite it? {} ",style(self.pass_name.display()).cyan().bold(),style("[y/N]").red().bold())?;
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
        let generator = PasswordGenerator::new(self.pass_len.unwrap_or(30) as usize).witch_no_symbol(self.no_symbols);
        let pass = generator.generate();
        if let Some(parent) = self.pass_name.parent() {
            let path = dirs::WS.store.join(parent);
            if !path.is_dir() {
                fs::create_dir_all(path)?;
            }
        }
        writeln!(&term,"{}",style(format!("The generated password for {} is:",self.pass_name.display())).bold())?;
        writeln!(&term,"{}",style(pass.as_str()).green().bold())?;
        let pass = key.encrypt(pass.as_bytes())?;
        fs::write(&path, pass.trim())?;
        store.git()?.add(&path)?;
        Ok(())
    }
}
