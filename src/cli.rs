use clap::{Parser, Subcommand};

use crate::command::*;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command to execute on destination host
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Init new pass storage
    Init(init::Cli),
    /// List passwords.
    Ls(ls::Cli),
    /// List passwords that match pass-names.
    Find(find::Cli),
    /// Show existing password and optionally put it on the clipboard.
    Show(show::Cli) ,
    /// Search for password files containing search-string when decrypted.
    Grep(grep::Cli),
    /// Insert new password.
    Insert(insert::Cli),
    /// Insert a new password or edit an existing password using nvim.
    Edit(edit::Cli),
    /// Generate a new password of pass-length (or 25 if unspecified) with optionally no symbols.
    Generate(generate::Cli),
    /// Remove existing password or directory, optionally forcefully.
    Rm(rm::Cli),
    /// Renames or moves old-path to new-path, optionally forcefully, selectively reencrypting.
    Mv(mv::Cli),
    /// Copy password
    Cp(cp::Cli),
    /// Key management
    Key(key::Cli),
    /// If the password store is a git repository, execute a git command specified by git-command-args.
    Git(git::Cli),
}