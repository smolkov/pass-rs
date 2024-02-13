use std::{path::PathBuf, sync::Mutex};
use std::env;


use serde::{Serialize, Deserialize};
use anyhow::Result;
use lazy_static::lazy_static;




lazy_static! {
    pub static ref CONFIG: Mutex<Config> = {
		let config = Config::default();	
        Mutex::new(config)
    };
}




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


pub fn get_gpg_key() -> String {
	CONFIG.lock().unwrap().gpg_key.to_owned()
}

pub fn store_dir() -> PathBuf {
	let home = dirs::home_dir().unwrap();
	home.join(".store")
}
