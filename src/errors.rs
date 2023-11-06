use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("expected tag to have a name")]
    MissingName,

    #[error("the amount of tags of the byte array must not exceed {}", i32::MAX)]
    ByteArrayTooBig,

    #[error("the length of the string (in bytes) must not exceed {}", u16::MAX)]
    StringTooBig,

    #[error("the amount of tags of the list must not exceed {}", i32::MAX)]
    ListTooBig,

    #[error("the amount of tags of the int array must not exceed {}", i32::MAX)]
    IntArrayTooBig,

    #[error("the amount of tags of the long array must not exceed {}", i32::MAX)]
    LongArrayTooBig,

    // TODO: improve this
    #[error("failed to parse NBT")]
    ParseError(nom::error::ErrorKind),

    #[error("NBT is incomplete")]
    Incomplete(nom::Needed),
}
