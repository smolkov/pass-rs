use std::fs;
use std::io::stdin;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use openssl::version::dir;

use crate::dirs;
use crate::password::Password;
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
        println!("Generate password {}", self.pass_name.display());
        let path = dirs::WS.store.join(&self.pass_name);
        let key = store.private_key()?;
        if path.exists() && ! self.force {
            println!("An entry already exists for test.com/sascha. Overwrite it? [y/N]");
            let mut user_input = String::new();
            stdin().read_line(&mut user_input)?;
            if !user_input.starts_with("y") {
                println!("exit because user input");
                return Ok(());
            }
        } else {
        }
        let generator = Password::new(self.pass_len.unwrap_or(30) as usize).witch_no_symbol(self.no_symbols);
        let pass = generator.generate();
        if let Some(parent) = self.pass_name.parent() {
            let path = dirs::WS.store.join(parent);
            if !path.is_dir() {
                fs::create_dir_all(path)?;
            }
        }
        println!("pass: {}",pass);
        let pass = key.encrypt(pass.as_bytes())?;
        fs::write(path, &pass.trim())?;
        println!("encrypted:{}",&pass);
        println!("decrypted:{}",key.decrypt(&pass)?);

        Ok(())
    }
}
