mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    match cli.command {
        args::Commands::Encode(args) => println!("encode {:?}", args),
        args::Commands::Decode(args) => println!("decode {:?}", args),
        args::Commands::Remove(args) => println!("remove {:?}", args),
        args::Commands::Print(args) => println!("print {:?}", args),
    }
    Ok(())
}
