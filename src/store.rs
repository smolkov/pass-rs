use anyhow::Result;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;
use crate::dirs::WS;
use crate::git::Git;
use crate::key::{PrivateKey};

static PRIVATE_KEY_FILE: &str = ".key";
static PUBLIC_KEY_FILE: &str = ".key.pub";
pub struct Store {
    path: PathBuf,
    #[allow(dead_code)]
    config: Config,
}

impl Store {
    pub fn new<P:AsRef<Path>>(path: P, config: Config) -> Result<Store> {
        let path = path.as_ref().to_path_buf();
        if !path.is_dir() {
            fs::create_dir_all(&path)?;
        }
        Ok(Store { path, config })
    }
    pub fn open(config: Config) -> Result<Store> {
        let path = WS.store.clone();
        Ok(Store { path, config })
    }
    pub fn git(&self) -> Result<Git> {
        Git::open(&self.path)
    }
    pub fn private_key(&self) -> Result<PrivateKey> {
        let key =
            PrivateKey::from_pem(fs::read_to_string(self.path.join(PRIVATE_KEY_FILE))?.as_bytes())?;
        Ok(key)
    }
    pub fn password(&self, path: &Path) -> PathBuf {
        self.path.join(path)
    }
    pub fn directory(&self) -> &Path {
        &self.path
    }
    pub fn update_keys(&self, key: &PrivateKey) -> Result<()> {
        let key_path = self.path.join(PRIVATE_KEY_FILE);
        if key_path.exists() {
            println!("Private key is already exist. Do you want to replace it(y/N): ");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            if !buffer.starts_with('y') {
                println!("Your answer is {} exit..", buffer);
                std::process::exit(1);
            }
        }
        fs::write(key_path, key.private_pem()?)?;
        let key_path = self.path.join(PUBLIC_KEY_FILE);
        fs::write(key_path, key.public_pem()?)?;
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        if !self.path.exists() {
            return true;
        }
        WalkDir::new(self.path.as_path())
            .into_iter()
            .filter_map(|e| e.ok())
            .skip(1)
            .count()
            == 0
    }
    pub fn files(&self) -> usize {
        WalkDir::new(self.path.as_path())
            .into_iter()
            .filter_map(|e| e.ok())
            .skip(1)
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_empty() {
        let store = Store::new("test", Config::default()).unwrap();
        assert!(store.is_empty());
        // println!("empty:{}",store.is_empty());
        // println!("files:{}",store.files());
        fs::remove_dir_all("test").unwrap();
    }
}
