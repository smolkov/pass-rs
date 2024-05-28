use std::fs;
use anyhow::Result;

use crate::dirs::WS;
use crate::config::Config;
use crate::key::PrivateKey;


pub struct Store {
	key: PrivateKey,
	config: Config
}



impl Store {
	pub fn open(config:Config) -> Result<Store> {
		let key = PrivateKey::from_pem(fs::read_to_string(WS.config.join(".key"))?.as_bytes())?;
		Ok(Store{key,config})
	}
}