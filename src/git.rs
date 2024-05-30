use std::path::Path;
use git2::Repository;
use anyhow::Result;

pub struct Git {
	repo: Repository,
}


impl Git {
	
	pub fn open(path:&Path) -> Result<Git> {
		let repo = if let Ok(repo)	= Repository::open(path) {
			repo
		}else {
			Repository::init(path)?
		};
		Ok(Git{repo})
	}
	pub fn clone(url: &str,path: &Path) -> Result<Git> {
		let repo = Repository::clone(url, path)?;
		Ok(Git{repo})
	}
	pub fn add(&self,path:&Path) -> Result<()>{
		let mut index = self.repo.index()?;
		index.add_path(path)?;
		Ok(())

	}

}