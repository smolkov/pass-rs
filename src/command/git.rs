use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::store::Store;

#[derive(Debug,Subcommand)] 
enum  Cmd {
    
}

#[derive(Debug, Parser)]
pub struct Cli {

}

impl Cli {
    pub fn run(&self, _store: &Store) -> Result<()> {
        Ok(())
    }
}
