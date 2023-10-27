use crate::byte_order::ByteOrder;
use crate::errors::Error;
use crate::parser;

pub type Name = Option<String>;

/// A tag is an individual part of the data tree. A tag consists of a name and
/// a payload. The name is absent when it is used within a [List].
///
/// The tag `TAG_End` is not available because it is handled by the program.
#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    /// A signed integral type. Sometimes used for booleans.
    Byte(Name, i8),

    /// A signed integral type.
    Short(Name, i16),

    /// A signed intefral type.
    Int(Name, i32),

    /// A signed integral type.
    Long(Name, i64),

    /// A signed floating point type.
    Float(Name, f32),

    /// A signed floating point type.
    Double(Name, f64),

    /// An array of bytes.
    ByteArray(Name, Vec<i8>),

    /// A UTF-8 string.
    String(Name, String),

    /// A list of tag payloads, without tag IDs or names.
    List(Name, Vec<Tag>),

    /// A list of fully formed tags, including their IDs, names, and payloads.
    Compound(Name, Vec<Tag>),

    /// An array of [Tag::Int]s.
    IntArray(Name, Vec<i32>),

    /// An array of [Tag::Long]s.
    LongArray(Name, Vec<i64>),
}

impl Tag {
    /// Returns a tag represented as bytes.
    pub fn to_bytes(&self, byte_order: ByteOrder) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        buf.extend(self.bytes_id(byte_order));
        buf.extend(self.bytes_name(byte_order)?);
        buf.extend(self.bytes_payload(byte_order)?);
        Ok(buf)
    }

    /// Returns a tag from bytes.
    #[cfg(feature = "read")]
    pub fn from_bytes(bytes: &[u8], byte_order: ByteOrder) -> Result<Self, Error> {
        Ok(parser::nbt(bytes, byte_order).expect("TODO").1)
    }

    /// Returns a vector of bytes with the length of the name and the name
    /// MUTF-8 encoded. This is an [std::result::Result::Err] when the name
    /// is absent.
    fn bytes_name(&self, byte_order: ByteOrder) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        match self {
            Tag::Byte(name, _)
            | Tag::Short(name, _)
            | Tag::Int(name, _)
            | Tag::Long(name, _)
            | Tag::Float(name, _)
            | Tag::Double(name, _)
            | Tag::ByteArray(name, _)
            | Tag::String(name, _)
            | Tag::List(name, _)
            | Tag::Compound(name, _)
            | Tag::IntArray(name, _)
            | Tag::LongArray(name, _) => {
                let n = name.as_ref().ok_or(Error::MissingName)?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().map_err(|_| Error::StringTooBig)?;
                buf.extend(byte_order.bytes(len));
                buf.extend(n);
            }
        };
        Ok(buf)
    }

    /// Returns the ID of the tag as a vector of bytes.
    fn bytes_id(&self, byte_order: ByteOrder) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(match *self {
            Tag::Byte(..) => byte_order.bytes(1_i8),
            Tag::Short(..) => byte_order.bytes(2_i8),
            Tag::Int(..) => byte_order.bytes(3_i8),
            Tag::Long(..) => byte_order.bytes(4_i8),
            Tag::Float(..) => byte_order.bytes(5_i8),
            Tag::Double(..) => byte_order.bytes(6_i8),
            Tag::ByteArray(..) => byte_order.bytes(7_i8),
            Tag::String(..) => byte_order.bytes(8_i8),
            Tag::List(..) => byte_order.bytes(9_i8),
            Tag::Compound(..) => byte_order.bytes(10_i8),
            Tag::IntArray(..) => byte_order.bytes(11_i8),
            Tag::LongArray(..) => byte_order.bytes(12_i8),
        });
        buf
    }

    /// Returns the payload of the tag as a vector of bytes.
    fn bytes_payload(&self, byte_order: ByteOrder) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        match self {
            Tag::Byte(_, payload) => buf.extend(byte_order.bytes(*payload)),
            Tag::Short(_, payload) => buf.extend(byte_order.bytes(*payload)),
            Tag::Int(_, payload) => buf.extend(byte_order.bytes(*payload)),
            Tag::Long(_, payload) => buf.extend(byte_order.bytes(*payload)),
            Tag::Float(_, payload) => buf.extend(byte_order.bytes(*payload)),
            Tag::Double(_, payload) => buf.extend(byte_order.bytes(*payload)),

            Tag::ByteArray(_, payload) => {
                // length of array
                let len: i32 = payload
                    .len()
                    .try_into()
                    .map_err(|_| Error::ByteArrayTooBig)?;
                buf.extend(byte_order.bytes(len));

                // content of array
                for byte in payload {
                    buf.extend(byte_order.bytes(*byte));
                }
            }

            Tag::String(_, payload) => {
                let string = mutf8::encode(payload.as_str());
                let len: u16 = string.len().try_into().map_err(|_| Error::StringTooBig)?;
                buf.extend(byte_order.bytes(len));
                buf.extend(string.into_owned());
            }

            Tag::List(_, payload) => {
                // tag ID
                if let Some(first) = payload.first() {
                    buf.extend(first.bytes_id(byte_order));
                }

                // length of list
                let len: i32 = payload.len().try_into().map_err(|_| Error::ListTooBig)?;
                buf.extend(byte_order.bytes(len));

                // content of list
                for byte in payload {
                    buf.extend(byte.bytes_payload(byte_order)?);
                }
            }

            Tag::Compound(_, payload) => {
                for tag in payload {
                    buf.extend(tag.to_bytes(byte_order)?);
                }
                buf.extend(byte_order.bytes(0_i8));
            }

            Tag::IntArray(_, payload) => {
                // length of array
                let len: i32 = payload
                    .len()
                    .try_into()
                    .map_err(|_| Error::IntArrayTooBig)?;
                buf.extend(byte_order.bytes(len));

                // content of array
                for int in payload {
                    buf.extend(byte_order.bytes(*int));
                }
            }

            Tag::LongArray(_, payload) => {
                // length of array
                let len: i32 = payload
                    .len()
                    .try_into()
                    .map_err(|_| Error::LongArrayTooBig)?;
                buf.extend(byte_order.bytes(len));

                // content of array
                for long in payload {
                    buf.extend(byte_order.bytes(*long));
                }
            }
        };
        Ok(buf)
    }
}

/// Wraps its tags in an unnamed compound.
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, nbt};
///
/// assert_eq!(
///     nbt![
///         Tag::Int(Some("foo".to_string()), 42),
///         Tag::Long(Some("bar".to_string()), 12)
///     ],
///     Tag::Compound(
///         Some("".to_string()),
///         vec![
///             Tag::Int(Some("foo".to_string()), 42),
///             Tag::Long(Some("bar".to_string()), 12)
///         ]
///     )
/// );
/// ```
#[macro_export]
macro_rules! nbt {
    ($($data:expr),* $(,)?) => {
        Tag::Compound(Some(String::new()), vec![$($data),*])
    };
}
