use std::path::PathBuf;
use std::io;
use std::fs;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

use crate::store::Store;
use crate::key::PrivateKey;
use crate::dirs;

/// Initialize new password storage and use gpg-id for encryption.


#[derive(Debug,Subcommand)]
enum  KeyCommand {
    Generate{
        /// Password length
        len: Option<u32>,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: KeyCommand,
}

impl Cli {
    pub fn run(&self,_store:&Store) -> Result<()> {
        match self.command {
           KeyCommand::Generate{len} => generate(_store,len)?
        } 
        Ok(())
    }
}


fn generate(_store:&Store,len : Option<u32>) -> Result<()> {
    let len = len.unwrap_or(4094);
    let privat_key = PrivateKey::generate_rsa(len)?;
    let key_path = dirs::WS.config.join(".key");
    if key_path.exists() {
        println!("Private key is already exist. Do you want to replace it(Y/n): ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if !buffer.starts_with("Y") {
            println!("Your answer is {} exit..",buffer);
            std::process::exit(1);
        }
    }
    fs::write(key_path, privat_key.private_pem()?)?;
    println!("{}",privat_key.private_pem()?);
    println!("{}",privat_key.public_pem()?);
    Ok(())
}