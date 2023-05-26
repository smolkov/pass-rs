use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = pass::cli::Args::parse();
    pass::command::run(cli.command,None)?;
    println!("Hello, new pass cli!");
    Ok(())
}
