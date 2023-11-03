use anyhow::Result;
use std::fs::File;
use std::io::BufReader;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;

pub fn encode(args: EncodeArgs);
pub fn decode(args: DecodeArgs);
pub fn remove(args: RemoveArgs);
pub fn print(args: PrintArgs) -> Result<()> {
    let file = File::open(args.file_path)?;
    let buffer = BufReader::new(file);
    let png = Png::try_from(buffer.buffer())?;
    println!("{}", png);
    Ok(())
}
