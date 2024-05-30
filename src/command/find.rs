use std::fs::{self, DirEntry};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};

use anyhow::Result;
use clap::Parser;
use console::{style, Term};
use walkdir::WalkDir;
static SEARCH_IN: &str = "/home/sascha/ws";

use crate::{dirs::WS, store::Store};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Pass names  
    pass_name: String,
}

impl Cli {
    pub fn run(&self, _store: &Store) -> Result<()> {
        let (sender, receiver) = channel();
        println!("Search Terms: {}", self.pass_name);
        find_name_in_directory(sender, PathBuf::from(SEARCH_IN), self.pass_name.clone());
        while let Ok(msg) = receiver.recv() {
            passwords_tree(PathBuf::from(&msg))?;
        }
        Ok(())
    }
}

pub fn find_name_in_directory(sender: Sender<String>, search_in: PathBuf, name: String) {
    std::thread::spawn(move || {
        for entry in fs::read_dir(&search_in)?.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let file_name = path.strip_prefix(&search_in)?.to_str().unwrap();
            // println!("{}",file_name);
            if file_name.contains(&name) {
                sender.send(path.to_str().unwrap().to_string())?;
            } else {
                if path.is_dir() && !path.is_symlink() {
                    find_name_in_directory(sender.clone(), path.clone(), name.clone());
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    });
}

pub fn passwords_tree(path: PathBuf) -> Result<()> {
    let term = Term::stdout();
    let ws_path = path.strip_prefix(SEARCH_IN)?;
    if path.is_dir() {
        writeln!(&term, "{}", style(ws_path.to_str().unwrap()).bold().blue())?;
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let ws_path = entry.path().strip_prefix(SEARCH_IN)?;
            writeln!(&term, "{}", ws_path.display())?;
        }
    }
    Ok(())
}
