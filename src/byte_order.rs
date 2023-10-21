use num_traits::ToBytes;

/// Java Edition uses big endian and Bedrock Edition uses little endian.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl ByteOrder {
    pub fn bytes(&self, num: impl ToBytes) -> Vec<u8> {
        match self {
            Self::BigEndian => num.to_be_bytes().as_ref().to_owned(),
            Self::LittleEndian => num.to_le_bytes().as_ref().to_owned(),
        }
    }
}
