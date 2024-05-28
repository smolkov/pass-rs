use clap::Parser;
use pass::config::Config;
use pass::store::Store;

fn main() -> anyhow::Result<()> {
    let cli = pass::cli::Args::parse();
    let config = Config::load()?;
    let store  = Store::open(config)?;
    pass::command::run(cli.command,&store)?;
    Ok(())
}
