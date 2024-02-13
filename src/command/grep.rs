use anyhow::Result;
use clap::Parser;


#[derive(Debug,Parser)]
pub struct Cli {
	/// Search string
	search_string: String,
}

impl Cli {
	pub fn run(&self)-> Result<()>{
	
		Ok(())
	}
}
