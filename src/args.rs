use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encodes a message into a chunk in the given file
    Encode(EncodeArgs),
    /// Decodes a message from a given chunk type in a file
    Decode(DecodeArgs),
    /// Removes a chunk from the given file
    Remove(RemoveArgs),
    /// Prints the file given
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to png file
    pub file_path: PathBuf,
    /// Chunk type to be injected into file
    pub chunk_type: String,
    /// Message encoded in chunk
    pub message: String,
    /// Optional output file with this chunk injected
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to png file
    pub file_path: PathBuf,
    /// Chunk type to be injected into file
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to png file
    pub file_path: PathBuf,
    /// Chunk type to be injected into file
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to png file
    pub file_path: PathBuf,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
