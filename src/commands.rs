use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use png_library::{Chunk, ChunkType, Png};

fn read_file_from_file_path(file_path: PathBuf) -> Result<Png> {
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    let png = Png::try_from(buffer.buffer())?;
    Ok(png)
}

pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = read_file_from_file_path(args.file_path)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);
    if let Some(output_file) = args.output_file {
        let file = File::open(output_file)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(png.as_bytes().as_slice())?;
    }
    Ok(())
}
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = read_file_from_file_path(args.file_path)?;
    let chunk = png
        .chunk_by_type(&args.chunk_type)
        .ok_or(anyhow!("Chunk doesnt have data"))?;
    let message = chunk.data_as_string()?;
    println!("Hidden message: {}", message);
    Ok(())
}
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
