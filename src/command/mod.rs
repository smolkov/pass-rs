pub mod cp;
pub mod edit;
pub mod find;
pub mod generate;
pub mod git;
pub mod grep;
pub mod init;
pub mod insert;
pub mod key;
pub mod ls;
pub mod mv;
pub mod rm;
pub mod show;

use crate::cli::Command;
use crate::store::Store;
use anyhow::Result;

pub fn run(cmd: Option<Command>, store: &Store) -> Result<()> {
    let cmd = cmd.unwrap();
    match cmd {
        Command::Cp(cli) => cli.run(store),
        Command::Edit(cli) => cli.run(store),
        Command::Find(cli) => cli.run(store),
        Command::Generate(cli) => cli.run(store),
        Command::Git(cli) => cli.run(store),
        Command::Grep(cli) => cli.run(store),
        Command::Init(cli) => cli.run(store),
        Command::Insert(cli) => cli.run(store),
        Command::Ls(cli) => cli.run(store),
        Command::Mv(cli) => cli.run(store),
        Command::Rm(cli) => cli.run(store),
        Command::Show(cli) => cli.run(store),
        Command::Key(cli) => cli.run(store),
    }
}
