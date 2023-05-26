use clap::Parser;

fn main() -> anyhow::Result<()> {
    let _cli = pass::cli::Args::parse();
    println!("Hello, new pass cli!");
    Ok(())
}
