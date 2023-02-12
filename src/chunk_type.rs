use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    #[allow(dead_code)]
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    #[allow(dead_code)]
    fn is_critical(&self) -> bool {
        (self.bytes[0] >> 5 & 1) == 0
    }

    #[allow(dead_code)]
    fn is_public(&self) -> bool {
        (self.bytes[1] >> 5 & 1) == 0
    }

    #[allow(dead_code)]
    fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] >> 5 & 1) == 0
    }

    #[allow(dead_code)]
    fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] >> 5 & 1) == 1
    }

    #[allow(dead_code)]
    fn is_valid(&self) -> bool {
        self.is_alphanumeric() && self.is_reserved_bit_valid()
    }

    fn is_alphanumeric(&self) -> bool {
        self.bytes.iter().all(u8::is_ascii_alphabetic)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("invalid chunk type")]
pub struct InvalidChunkType;

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = InvalidChunkType;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType { bytes };
        chunk_type
            .is_valid()
            .then_some(chunk_type)
            .ok_or(InvalidChunkType)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChunkTypeError {
    #[error("string must contain exactly 4 bytes")]
    InvalidLength,
    #[error("string is not alphanumeric")]
    NotAlphanumeric,
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .as_bytes()
            .try_into()
            .map_err(|_| ChunkTypeError::InvalidLength)?;
        let chunk_type = ChunkType { bytes };
        chunk_type
            .is_alphanumeric()
            .then_some(chunk_type)
            .ok_or(ChunkTypeError::NotAlphanumeric)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes).unwrap())
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
