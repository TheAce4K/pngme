use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;

fn read_file_from_file_path(file_path: PathBuf) -> Result<Png> {
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    let png = Png::try_from(buffer.buffer())?;
    Ok(png)
}

pub fn encode(args: EncodeArgs);
pub fn decode(args: DecodeArgs);
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = read_file_from_file_path(args.file_path)?;
    png.remove_chunk(&args.chunk_type)?;
    Ok(())
}
pub fn print(args: PrintArgs) -> Result<()> {
    let png = read_file_from_file_path(args.file_path)?;
    println!("{}", png);
    Ok(())
}
