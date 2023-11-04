use crate::chunk_type::ChunkType;
use anyhow::{anyhow, bail, Result};
use std::{
    fmt,
    io::{BufReader, Read},
};

use crc::{Crc, CRC_32_ISO_HDLC};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    length: u32,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;
        let hasher = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = hasher.checksum(crc_data.as_slice());
        Chunk {
            chunk_type,
            data,
            length,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.to_owned()) {
            Ok(string) => Ok(string),
            Err(_) => Err(anyhow!("could not convert from string")),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .cloned()
            .chain(self.chunk_type().bytes().iter().cloned())
            .chain(self.data().iter().cloned())
            .chain(self.crc().to_be_bytes().iter().cloned())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(value);
        let mut length: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut length)?;
        let length = u32::from_be_bytes(length);
        let mut chunk_data: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut chunk_data)?;
        let chunk_type = ChunkType::try_from(chunk_data)?;
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data)?;
        let mut crc: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut crc)?;
        let crc = u32::from_be_bytes(crc);
        let crc_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let valid_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(crc_data.as_slice());
        if crc != valid_crc {
            bail!("Crc is not valid")
        }
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chunk: data: {:?}, chunk_type: {}, length: {}, crc: {}",
            self.data, self.chunk_type, self.length, self.crc
        )
    }
}
