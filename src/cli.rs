use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command to execute on destination host
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize new password storage and use gpg-id for encryption.
    Init {
        /// Storage path (optional)
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// GPG id ( optional)
        gpg_id: Option<String>,
    },
    /// List passwords.
    Ls {
        /// subfolder
        subfolder: Option<PathBuf>,
    },
    /// List passwords that match pass-names.
    Find {
        /// Pass names  
        pass_name: String,
    },
    /// Show existing password and optionally put it on the clipboard.
    Show {
        /// Put on the clipboard, it will be cleared in 45 seconds.
        #[arg(short, long, default_value_t = false)]
        clip: bool,
        /// password name
        pass_name: Option<String>,
    },
    /// Search for password files containing search-string when decrypted.
    Grep {
        search_string: String,
    },
    /// Insert new password.
    Insert {
        /// Force insert new password
        #[arg(short, long, default_value_t = false)]
        force: bool,
        /// Password name
        pass_name: String,
    },
    /// Insert a new password or edit an existing password using mvim.
    Edit {
        /// Password name 
        pass_name: String
    },
    /// Generate a new password of pass-length (or 25 if unspecified) with optionally no symbols.
    Generate{
       /// Force insert new password
       #[arg(short, long, default_value_t = false)]
       no_symbols: bool,
       /// Optionally put on the clipboard, it will be cleared in 45 seconds.
       #[arg(short, long, default_value_t = false)]
       clip: bool,
       /// Force generate 
       #[arg(short, long, default_value_t = false)]
       force: bool, 
       /// Password name
       pass_name: String,
       /// Password length
       pass_len: Option<u8>
    },
    /// Remove existing password or directory, optionally forcefully.
    Rm {
         /// Recursive to rm directory
         #[arg(short, long, default_value_t = false)]
         recursive: bool, 
    },
    /// Renames or moves old-path to new-path, optionally forcefully, selectively reencrypting.
    Mv {
       /// Force insert new password
       #[arg(short, long, default_value_t = false)]
       force: bool, 
       /// Old path
       old_path: PathBuf,
       /// Moved to new path
       new_path: PathBuf
    },
    Cp {
       /// Source path
       src_path: PathBuf,
       /// Destination path
       dest_path: PathBuf 
    },
    /// If the password store is a git repository, execute a git command specified by git-command-args.
    Git{

    }

}
