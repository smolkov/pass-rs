use anyhow::Result;
use git2::{Repository, Signature,RemoteCallbacks,Cred};
use std::path::Path;

const REMOTE: &str = "origin";
// const BRANCH: &str = "main";

pub struct Git {
    pub repo: Repository,
}

impl Git {
    pub fn open(path: &Path) -> Result<Git> {
        let repo = if let Ok(repo) = Repository::open(path) {
            repo
        } else {
            Repository::init(path)?
        };
        Ok(Git { repo })
    }
    pub fn clone(url: &str, path: &Path) -> Result<Git> {
        let repo = Repository::clone(url, path)?;
        Ok(Git { repo })
    }
    pub fn add<P:AsRef<Path>>(&self, path: P) -> Result<()> {
		let path = path.as_ref().strip_prefix(self.repo.workdir().unwrap()).unwrap_or(path.as_ref());
        let mut index = self.repo.index()?;
        index.add_path(path.as_ref())?;
        index.write()?;
        Ok(())
    }
    pub fn push(&self) -> Result<()> {
        let mut remote = self.repo.find_remote(REMOTE)?;
		let mut callbacks = RemoteCallbacks::new();
		callbacks.credentials(|_url, username_from_url, _allowed_types| {
			Cred::ssh_key(
		     username_from_url.unwrap(),
		     None,
		     std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or(String::from(".")))),
		     None,
		   )
		 });
		let mut push_options = git2::PushOptions::new();
		push_options.remote_callbacks(callbacks);
		let refspecs: &[&str] = &["refs/heads/main:refs/heads/main"]; // Adjust branch names as needed
        remote.push(refspecs, Some(&mut push_options))?;
        Ok(())
    }
    pub fn commit(&self, message: &str) -> Result<()> {
        // Create a commit signature
        let signature = Signature::now("name", "your.email@example.com")?;
        let mut index = self.repo.index()?;
        // Create a commit
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        // Get the current HEAD reference
        let parent_commit = self.repo.head()?.peel_to_commit()?;
        // Commit the changes
        self.repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
				message,
                &tree,
                &[&parent_commit],
            )
            .expect("Failed to create commit");
        Ok(())
    }
    pub fn add_remote(&self, remote_url: &str) -> Result<()> {
        self.repo.remote(REMOTE, remote_url)?;
        Ok(())
    }
}
