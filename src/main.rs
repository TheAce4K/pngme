extern crate png_library;
pub mod args;
mod commands;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    match cli.command {
        args::Commands::Encode(args) => {
            println!("encode {:#?}", args);
            commands::encode(args)?;
        }
        args::Commands::Decode(args) => {
            println!("decode {:?}", args);
            commands::decode(args)?;
        }
        args::Commands::Remove(args) => {
            println!("remove {:?}", args);
            commands::remove(args)?;
        }
        args::Commands::Print(args) => {
            println!("print {:?}", args);
            commands::print(args)?;
        }
    }
    Ok(())
}
