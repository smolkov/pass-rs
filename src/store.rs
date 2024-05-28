use anyhow::Result;
use std::fs;
use std::io;
use std::path::{PathBuf,Path};

use crate::config::Config;
use crate::dirs;
use crate::dirs::WS;
use crate::key::{PrivateKey, PublicKey};

static PRIVATE_KEY_FILE: &str = ".key";
static PUBLIC_KEY_FILE: &str = ".key.pub";
pub struct Store {
	path: PathBuf,
    config: Config,
}

impl Store {
    pub fn open(config:Config) -> Result<Store> {
		let path = WS.store.clone();	
        Ok(Store { path,config })
    }
    pub fn private_key(&self) -> Result<PrivateKey> {
        let key = PrivateKey::from_pem(fs::read_to_string(self.path.join(PRIVATE_KEY_FILE))?.as_bytes())?;
        Ok(key)
    }
    pub fn public_key(&self) -> Result<PublicKey> {
        let key = PublicKey::from_pem(fs::read_to_string(self.path.join(PUBLIC_KEY_FILE))?.as_bytes())?;
        Ok(key)
    }
	pub fn path(&self) -> &Path {
		self.path.as_path()
	}
    pub fn update_keys(&self, key: &PrivateKey) -> Result<()> {
        let key_path = self.path.join(PRIVATE_KEY_FILE);
        if key_path.exists() {
            println!("Private key is already exist. Do you want to replace it(Y/n): ");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            if !buffer.starts_with("Y") {
                println!("Your answer is {} exit..", buffer);
                std::process::exit(1);
            }
        }
        fs::write(key_path, key.private_pem()?)?;
        let key_path = self.path.join(PUBLIC_KEY_FILE);
        fs::write(key_path, key.public_pem()?)?;
        Ok(())
    }
}
