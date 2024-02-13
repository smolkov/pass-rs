pub mod cp;
pub mod edit;
pub mod find;
pub mod generate;
pub mod git;
pub mod grep;
pub mod init;
pub mod insert;
pub mod ls;
pub mod mv;
pub mod rm;
pub mod show;

use crate::cli::Command;
use anyhow::Result;

pub fn run(cmd: Option<Command>, _name: Option<String>) -> Result<()> {
    let cmd = cmd.unwrap();
    match cmd {
        Command::Cp(cli) => cli.run(),
        Command::Edit(cli) => cli.run(),
        Command::Find(cli) => cli.run(),
        Command::Generate(cli) => cli.run(),
        Command::Git(cli)=> cli.run(),
        Command::Grep(cli) => cli.run(),
        Command::Init(cli) => cli.run(),
        Command::Insert(cli) => cli.run(),
        Command::Ls(cli) => cli.run(),
        Command::Mv(cli) => cli.run(),
        Command::Rm(cli) => cli.run(),
        Command::Show(cli) => cli.run(),
    }
}
