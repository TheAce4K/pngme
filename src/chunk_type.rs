#![allow(unused_variables)]
use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, bail, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() || !self._is_ascii_ok() {
            return false;
        }
        return true;
    }

    fn _is_ascii_ok(&self) -> bool {
        for byte in self.bytes.iter() {
            if !byte.is_ascii_alphabetic() {
                return false;
            }
        }
        return true;
    }

    #[allow(dead_code)]
    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    #[allow(dead_code)]
    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    #[allow(dead_code)]
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    // Required method
    fn try_from(value: [u8; 4]) -> Result<Self> {
        let chunk = ChunkType { bytes: value };
        if chunk.is_valid() {
            Ok(chunk)
        } else {
            Err(anyhow!(
                "Cant convert to chunk type, chunk is not valid. Chunk: {}",
                chunk
            ))
        }
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let bytes: [u8; 4] = match s.bytes().collect::<Vec<u8>>().try_into() {
            Ok(bytes) => bytes,
            Err(_) => bail!("Cant convert to bytes array"),
        };

        let chunk = ChunkType { bytes };
        if chunk._is_ascii_ok() {
            Ok(chunk)
        } else {
            bail!("Not a valid string")
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string =
            String::from_utf8(self.bytes.to_vec()).expect("all bytes can be converted to utf8");
        write!(f, "{}", string)
    }
}
