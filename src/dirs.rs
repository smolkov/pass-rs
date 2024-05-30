pub use std::env::*;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use std::fs;

pub static HOME: Lazy<PathBuf> = Lazy::new(||dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")));
pub static WS: Lazy<Dirs> = Lazy::new(Dirs::new);





pub struct Dirs {
	pub store: PathBuf,
	pub config: PathBuf,
}

impl Dirs {
	pub fn new() -> Dirs {
		let home = var("HOME").map(PathBuf::from).unwrap_or(PathBuf::from("."));
		let store = home.join(".passwords");
		let config = home.join(".config/pass");
		if !store.is_dir() {
			fs::create_dir_all(&store).expect("something is wrong create passwords store directory failed");
		}
		if !config.is_dir() {
			fs::create_dir_all(&config).expect("something is wrong create passwords config directory failed");

		}
		Dirs{store,config}
	}
}

impl Default for Dirs {
	fn default() -> Self {
		Dirs::new()
	}
}