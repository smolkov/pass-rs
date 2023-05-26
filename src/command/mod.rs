mod cp;
mod edit;
mod find;
mod generate;
mod git;
mod grep;
mod init;
mod insert;
mod ls;
mod mv;
mod rm;
mod show;

use anyhow::Result;
use crate::cli::Command;


pub fn run(cmd: Option<Command>,_name:Option<String>) -> Result<()>{
	let cmd = cmd.unwrap();	
	match cmd {
	    Command::Cp {src,dest }	=> cp::run(src, dest),
		Command::Edit { pass_name } => edit::run(pass_name),
		Command::Find { pass_name } => find::run(pass_name),
		Command::Generate { no_symbols, clip, force, pass_name, pass_len } => generate::run(no_symbols, clip, force, pass_name, pass_len),
		Command::Git {  } => git::run(),
		Command::Grep { search_string } => grep::run(search_string),
		Command::Init { path, gpg_id } => init::run(path,gpg_id),
		Command::Insert { force, pass_name } => insert::run(force, pass_name),
		Command::Ls { subfolder } => ls::run(subfolder),
		Command::Mv { force, src, dest } => mv::run(force,src,dest),
		Command::Rm { recursive ,pass_name} => rm::run(recursive, pass_name),
		Command::Show { clip, pass_name } => show::run(clip, pass_name),
	}

}
