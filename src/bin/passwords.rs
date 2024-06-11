use clap::Parser;
use password::config::Config;
use password::store::Store;

fn main() -> anyhow::Result<()> {
    let cli = password::cli::Args::parse();
    let config = Config::load()?;
    let store  = Store::new(password::dirs::STORE.as_path(),config)?;
    password::command::run(cli.command,&store)?;
    Ok(())
}
