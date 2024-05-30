use anyhow::Result;
use clap::{Parser, Subcommand};
use git2::Repository;

use crate::git::Git;
use crate::key::PrivateKey;
use crate::store::Store;

#[derive(Debug, Parser)]
struct New {
    /// n
    #[arg(short, long, default_value_t = 4095)]
    len: u32,
}

#[derive(Debug, Parser)]
struct Checkout {
    url: String,
}

/// Initialize new password storage and use gpg-id for encryption.
#[derive(Debug, Subcommand)]
enum Command {
    /// create new password store
    New(New),
    /// checkout remote password store
    Checkout(Checkout),
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        match &self.cmd {
            Command::New(cmd) => new(store, cmd),
            Command::Checkout(cmd) => checkout(store, cmd),
        }
    }
}

fn new(store: &Store, new: &New) -> Result<()> {
    let privat_key = PrivateKey::generate_rsa(new.len)?;
    store.update_keys(&privat_key)?;
    println!("{}", privat_key.private_pem()?);
    println!("{}", privat_key.public_pem()?);
    Ok(())
}

fn checkout(store: &Store, checkout: &Checkout) -> Result<()> {
    match Git::clone(&checkout.url, store.directory()) {
        Ok(_) => {}
        Err(e) => panic!("failed to clone: {}", e),
    }
    Ok(())
}
