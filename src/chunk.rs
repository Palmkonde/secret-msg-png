use std::convert::TryFrom;
use std::fmt;
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub struct Chunk { 
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    
    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return Err("Chunk data must be at least 12 bytes long".into());
        }     
        else {
            let length: u32 = u32::from_be_bytes(value[0..4].try_into().unwrap());
            
            let total_length: usize = 4 + 4 + length as usize + 4;
            if value.len() != total_length {
                return Err(format!("Chunk data length mismatch: expected {}, got {}", total_length, value.len()).into())
            }
            
            let bytes_array: [u8; 4] = value[4..8].try_into().unwrap();
            let chunk_type: ChunkType = ChunkType::try_from(bytes_array).unwrap();
            
            let end_data: usize = 8 + length as usize;
            let data: Vec<u8> = value [8..end_data].to_vec();
 
            let crc: u32 = u32::from_be_bytes(value[end_data..end_data + 4].try_into().unwrap());
            
            let expected_crc = PNG_CRC.checksum(
                &chunk_type.bytes()
                    .iter()
                    .chain(data.iter())
                    .copied()
                    .collect::<Vec<u8>>(),    
            );
            
            if crc != expected_crc {
                return Err(format!("CRC mismatch: expected {}, got {}", expected_crc, crc).into());
            }

            return Ok(Chunk {
                length,
                chunk_type,
                data,
                crc,
            })
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Length: {}\n", self.length)?;
        write!(f, "Chunk Type: {}\n", self.chunk_type)?;
        write!(f, "Data: {:?}\n", self.data)?;
        write!(f, "CRC: {}\n", self.crc)?;
        Ok(())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;
        
        let crc = PNG_CRC.checksum(
            &chunk_type.bytes()
                .iter()
                .chain(data.iter())
                .copied()
                .collect::<Vec<u8>>(),
        );

        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }
    
    pub fn length(&self) -> u32 { self.length }
    pub fn chunk_type(&self) -> &ChunkType { &self.chunk_type }
    pub fn data(&self) -> &[u8] { &self.data }
    pub fn crc(&self) -> u32 { self.crc }
    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data.clone())
            .map_err(|e| format!("Failed to convert chunk data to string :{}", e).into())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.length.to_be_bytes().iter());
        bytes.extend(self.chunk_type.bytes().iter());
        bytes.extend(self.data.iter());
        bytes.extend(self.crc.to_be_bytes().iter());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}