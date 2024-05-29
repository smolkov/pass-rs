use std::fs::{self,DirEntry};

use anyhow::Result;
use clap::Parser;
// use walkdir::WalkDir;

use crate::{dirs::WS, store::Store};
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Pass names  
    pass_name: String,
}

impl Cli {
    pub fn run(&self, store: &Store) -> Result<()> {
        // for entry in WalkDir::new("/home/sascha/")
        //     .follow_links(true)
        //     .into_iter()
        //     .filter_map(|e| e.ok())
        // {
        //     let f_name = entry.file_name().to_string_lossy();
        //     if f_name.contains(&self.pass_name) {
        //         println!("{}", f_name);
        //     }
        // }
        for entry in fs::read_dir("/home/sascha/")?.into_iter().filter_map(|e|e.ok()) {
           let path = entry.path();
        //    let name = entry.file_name().to_string_lossy();
            println!("{}",path.display()); 
        }

        Ok(())
    }
}


fn print_file_path(entry: &DirEntry) {

}