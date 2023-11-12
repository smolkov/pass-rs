use serde::{Serialize, Deserialize};
use anyhow::Result;





#[derive(Debug,Serialize,Deserialize)]
pub struct Config {
	pub gpg_key: String,
}


impl Default for Config {
	fn default() -> Self {
		Config { gpg_key: "".to_owned() }
	}
}

pub fn load() -> Result<Config> {
	Ok(Config::default())
}