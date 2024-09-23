
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

use crate::store::Store;
use crate::key::PrivateKey;

/// Initialize new password storage and use gpg-id for encryption.


#[derive(Debug,Parser)]
struct KeyGenerate {
    /// Key len
    len: Option<u32>,
}

#[derive(Debug,Subcommand)]
enum  KeyCommand {
    Generate(KeyGenerate),
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: KeyCommand,
}

impl Cli {
    pub fn run(&self,store:&Store) -> Result<()> {
        match &self.command {
           KeyCommand::Generate(gen) => gen.generate(store)?
        } 
        Ok(())
    }
}


impl KeyGenerate {
    fn generate(&self, store: &Store) -> Result<()> {
        let len = self.len.unwrap_or(4094);
        let private_key = PrivateKey::generate_rsa(len)?;
        store.update_keys(&private_key)?;
        println!("{}",private_key.private_pem()?);
        println!("{}",private_key.public_pem()?);
        Ok(())
    }
}