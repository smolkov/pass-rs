use std::io::stdin;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::password::Generator;

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
    pub fn run(&self) -> Result<()> {
        println!("Generate password {}", self.pass_name.display());
        // TODO:
        // [ ] join store path + name.
        // [x] check if exist replace or exit.
        // [ ] if not exist create folder.
        // [x] generate password.
        // [ ] write password.
        // [ ] encrypt file.
        if self.pass_name.exists() {
            println!("An entry already exists for test.com/sascha. Overwrite it? [y/N]");
            let mut user_input = String::new();
            stdin().read_line(&mut user_input)?;
            if user_input != "y" {
                return Ok(());
            }
        } else {
        }
        let generator = Generator::new(self.pass_len.unwrap_or(30) as usize);

        Ok(())
    }
}
