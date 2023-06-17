use crate::{Result,Error};
use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(PartialEq,Eq, PartialOrd, Ord,Debug,Clone)]
/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
pub struct ChunkType{
    code:[u8;4]
}

impl ChunkType{
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8;4]{
        self.code
    }

    #[allow(dead_code)]
    /// Returns the property state of the first byte as described in the PNG spec
    fn is_critical(&self)->bool{
        (self.code[0] & 0b00100000) != 0b00100000
    }

    #[allow(dead_code)]
    /// Returns the property state of the second byte as described in the PNG spec
    fn is_public(&self)->bool{
        (self.code[1] & 0b00100000) != 0b00100000
    }

    /// Returns the property state of the third byte as described in the PNG spec
    fn is_reserved_bit_valid(&self)->bool{
        (self.code[2] & 0b00100000) != 0b00100000
    }
    
    #[allow(dead_code)]
    /// Returns the property state of the fourth byte as described in the PNG spec
    fn is_safe_to_copy(&self)->bool{
        (self.code[3] & 0b00100000) == 0b00100000
    }

    // Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool{
        self.is_reserved_bit_valid() && self
        .bytes()
        .iter()
        .all(|&e| ChunkType::is_valid_byte(e)) 
    }

    #[allow(dead_code)]
    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }


}

impl TryFrom<[u8;4]> for ChunkType{
    type Error = Error;
    fn try_from(value: [u8;4]) -> Result<Self> {
        Ok(Self{ code: value })
    }
}

impl FromStr for ChunkType{
    type Err = Error;
    fn from_str(str: &str) -> Result<Self> {
        let str_bytes = str.as_bytes();
        if str_bytes.len() != 4{
            return Err(Box::new(ChunkTypeError::LengthError(str_bytes.len())));
        }
        if !str_bytes
            .iter()
            .all(|&b| ChunkType::is_valid_byte(b)){
                return Err(Box::new(ChunkTypeError::IllegalCharacter));
            }

        let code = [str_bytes[0],str_bytes[1],str_bytes[2],str_bytes[3]];
        Ok(Self{code})   
    }
}

impl Display for ChunkType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = std::str::from_utf8(&self.code).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}


#[derive(Debug)]
pub enum ChunkTypeError {
    LengthError(usize),
    IllegalCharacter,
}

impl std::error::Error for ChunkTypeError {}
impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::LengthError(length) => write!(f,"Expected 4 bytes but found {length} "),
            ChunkTypeError::IllegalCharacter => write!(f, "Contains non alphabetic characters")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}