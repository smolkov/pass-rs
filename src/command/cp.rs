use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;

use crate::store::Store;




#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Source path
	src: PathBuf,
	/// Destination path
	dest: PathBuf 
}




impl Cli {
	pub fn run(&self,store:&Store) -> Result<()> {
		std::fs::copy(store.password(&self.src),store.password(self.dest.as_path()))?;
		Ok(())
	}
}