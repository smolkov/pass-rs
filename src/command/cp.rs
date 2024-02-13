use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;



#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Source path
	src: PathBuf,
	/// Destination path
	dest: PathBuf 
}




impl Cli {
	pub fn run(&self) -> Result<()> {
		std::fs::copy(self.src.as_path(),self.dest.as_path())?;
		Ok(())
	}
}