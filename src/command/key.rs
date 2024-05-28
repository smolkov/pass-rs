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
    pub fn run(&self,store:&Store) -> Result<()> {
        match self.command {
           KeyCommand::Generate{len} => generate(store,len)?
        } 
        Ok(())
    }
}


fn generate(store:&Store,len : Option<u32>) -> Result<()> {
    let len = len.unwrap_or(4094);
    let privat_key = PrivateKey::generate_rsa(len)?;
    store.update_keys(&privat_key)?;
    println!("{}",privat_key.private_pem()?);
    println!("{}",privat_key.public_pem()?);
    Ok(())
}