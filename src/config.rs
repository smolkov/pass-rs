use std::{path::PathBuf, sync::RwLock};
use std::env;


use serde::{Serialize, Deserialize};
use anyhow::Result;




// static CONFIG: RwLock<Config>  = RwLock::new(Config::default());



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
