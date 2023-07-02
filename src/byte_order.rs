/// Java Edition uses big endian and Bedrock Edition uses little endian.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}
