use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::store::Store;

#[derive(Debug,Subcommand)] 
enum  Cmd {
   Remote{
    remote_url:String,
   },
   Push,
   Commit{
    message:String,
   },
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd:Cmd
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        match &self.cmd {
            Cmd::Remote { remote_url} => change_remote(store, remote_url.as_str())?,
            Cmd::Commit { message} => commit_all(store, message)?,
            Cmd::Push => push(store)?,
        }
        Ok(())
    }
}

fn change_remote(store:&Store, remote_url: &str) -> Result<()> {
    store.git()?.add_remote(remote_url)?;
    Ok(())
}

fn commit_all(store:&Store, message: &str) -> Result<()> {
    store.git()?.commit(message)?;
    Ok(())
}

fn push(store:&Store) -> Result<()> {
    store.git()?.push()?;
    Ok(())
}