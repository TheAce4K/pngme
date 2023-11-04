use std::fmt;

use crate::chunk::Chunk;
use anyhow::{bail, Ok, Result};

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png { chunks }
    }
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        match self
            .chunks
            .iter()
            .position(|chunk| chunk.chunk_type().bytes() == chunk_type.as_bytes())
        {
            Some(index) => Ok(self.chunks.swap_remove(index)),
            None => bail!("Cant find chunk in Png"),
        }
    }

    pub fn header(&self) -> &[u8; 8] {
        return &Self::STANDARD_HEADER;
    }

    pub fn chunks(&self) -> &[Chunk] {
        return &self.chunks;
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        return self
            .chunks
            .iter()
            .find(|chunk| chunk.chunk_type().bytes() == chunk_type.as_bytes());
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.header()
            .iter()
            .cloned()
            .chain(self.chunks.iter().flat_map(|chunk| chunk.as_bytes()))
            .collect()
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Png: {{ ")?;
        writeln!(f, "  header: {:?}", self.header())?;
        writeln!(f, "  chunks: [")?;
        for chunk in self.chunks() {
            write!(f, "{}", chunk)?;
        }
        write!(f, "]}}")
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        let header = &value[..8];
        if header != Self::STANDARD_HEADER {
            bail!("Header is incorrect")
        }
        let mut chunks = vec![];
        let mut remaining_bytes = &value[8..];
        while !remaining_bytes.is_empty() {
            let chunk = Chunk::try_from(remaining_bytes)?;
            // a chunk is chunk.length() + 12 bytes long because of the length of
            // the chunk type is 4 the crc is 4 and the length variable is 4
            remaining_bytes = &remaining_bytes[chunk.length() as usize + 12..];
            chunks.push(chunk);
        }
        Ok(Png::from_chunks(chunks))
    }
}
