use crate::chunk_type::ChunkType;
use crate::{Result, Error};

use std::fmt::Display;
use std::io::{BufReader, Read};
use crc::CRC_32_ISO_HDLC;

#[derive(Debug)]
pub struct Chunk{
    chunk_type:ChunkType,
    chunk_data:Vec<u8>,
}

impl Chunk{

    /// Creates a new instance of `Chunk`
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self {chunk_type,chunk_data:data}
    }

     /// The length of the data portion of this chunk.
     pub fn length(&self) -> u32 {
        self.chunk_data.len() as u32
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    #[allow(dead_code)]
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        let bytes:Vec<u8> = self
                    .chunk_type
                    .bytes()
                    .iter()
                    .chain(self.chunk_data.iter())
                    .copied()
                    .collect();

        let png_crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc = png_crc.checksum(&bytes);
        crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        let data_as_string = std::str::from_utf8(&self.chunk_data)?.to_string();
        Ok(data_as_string)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        self
            .length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk{
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return  Err(Box::new(ChunkError::SmallInput));
        }
        let mut reader = BufReader::new(value);
        let mut buffer:[u8;4] = [0,0,0,0];

        reader.read_exact(&mut buffer)?;
        let data_length: u32 = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;
        
        if !chunk_type.is_valid(){
            return Err(Box::new(ChunkError::InvalidChunkType));
        }

       let mut data_buffer = vec![0;data_length as usize];
        reader.read_exact(&mut data_buffer)?;
        let chunk_data = data_buffer;

        reader.read_exact(&mut buffer)?;
        let crc_bytes = u32::from_be_bytes(buffer);

        let new_chunk = Self{
            chunk_type,
            chunk_data,
        };
        let crc = new_chunk.crc();
        let given_crc = crc_bytes;

        if crc!=given_crc {
             return Err(Box::new(ChunkError::InvalidCrc));
        }

        Ok(new_chunk)

    }
}

impl Display for Chunk{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} ",self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}


#[derive(Debug)]
pub enum ChunkError {
    SmallInput,
    InvalidCrc,
    InvalidChunkType,
}

impl std::error::Error for ChunkError {}

impl std::fmt::Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChunkError::SmallInput=> write!(f, "At least 12 bytes needeed to create a Chunk"),
            ChunkError::InvalidCrc => write!(f,"CRC of chunk doesnot match with calculated CRC"),
            ChunkError::InvalidChunkType => write!(f, "Invalid chunk type"),
        }
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