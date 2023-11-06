use crate::byte_order::ByteOrder;
use crate::errors::Error;

/// Maximum amount of items in an array that are used for pretty formatting.
pub const ABBREVIATE_ARRAY_SIZE: u64 = 50;

/// String inserted in front of nested items for pretty formatting.
pub const INDENT: &'static str = "   "; // three spaces

pub type Name = Option<String>;

/// A tag is an individual part of the data tree. A tag consists of a name and
/// a payload. The name is absent when it is used within a [`Tag::List`].
///
/// The tag `TAG_End` is not available because it is handled by the program.
#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Tag {
    /// A signed integral type. Sometimes used for booleans.
    Byte(Name, i8) = 1,

    /// A signed integral type.
    Short(Name, i16) = 2,

    /// A signed intefral type.
    Int(Name, i32) = 3,

    /// A signed integral type.
    Long(Name, i64) = 4,

    /// A signed floating point type.
    Float(Name, f32) = 5,

    /// A signed floating point type.
    Double(Name, f64) = 6,

    /// An array of bytes.
    ByteArray(Name, Vec<i8>) = 7,

    /// A UTF-8 string.
    String(Name, String) = 8,

    /// A list of tag payloads, without tag IDs or names.
    List(Name, Vec<Tag>) = 9,

    /// A list of fully formed tags, including their IDs, names, and payloads.
    Compound(Name, Vec<Tag>) = 10,

    /// An array of [Tag::Int]s.
    IntArray(Name, Vec<i32>) = 11,

    /// An array of [Tag::Long]s.
    LongArray(Name, Vec<i64>) = 12,
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
        Ok(crate::parser::nbt(bytes, byte_order)
            .map_err(|e| match e {
                nom::Err::Incomplete(needed) => Error::Incomplete(needed),
                nom::Err::Error(e) | nom::Err::Failure(e) => Error::ParseError(e.code),
            })?
            .1)
    }

    /// Returns tag's name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mcnbt::Tag;
    ///
    /// assert_eq!(
    ///     Tag::Byte(Some("foo".to_string()), 42).name(),
    ///     "TAG_Byte"
    /// );
    pub fn name(&self) -> &'static str {
        match self {
            Tag::Byte(_, _) => "TAG_Byte",
            Tag::Short(_, _) => "TAG_Short",
            Tag::Int(_, _) => "TAG_Int",
            Tag::Long(_, _) => "TAG_Long",
            Tag::Float(_, _) => "TAG_Float",
            Tag::Double(_, _) => "TAG_Double",
            Tag::ByteArray(_, _) => "TAG_Byte_Array",
            Tag::String(_, _) => "TAG_String",
            Tag::List(_, _) => "TAG_List",
            Tag::Compound(_, _) => "TAG_Compound",
            Tag::IntArray(_, _) => "TAG_Int_Array",
            Tag::LongArray(_, _) => "TAG_Long_Array",
        }
    }

    /// Returns a pretty representation of the tag.
    ///
    /// The format matches the style used in the original specification and in
    /// many other NBT parsers with two different: tags that store an array of
    /// values will be abbreviated if its length is greater than
    /// [ABBREVIATE_ARRAY_SIZE] and if the name is not present `(None)` is omitted.
    pub fn pretty(&self) -> String {
        let mut result = String::new();
        match self {
            Tag::Byte(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {}", payload));
            }
            Tag::Short(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {}", payload));
            }
            Tag::Int(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {}", payload));
            }
            Tag::Long(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {}", payload));
            }
            Tag::Float(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {}", payload));
            }
            Tag::Double(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {}", payload));
            }
            Tag::ByteArray(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {} entries\n{{\n", payload.len()));
                let mut bytes = payload.into_iter();
                for _ in 0..ABBREVIATE_ARRAY_SIZE {
                    match bytes.next() {
                        Some(value) => {
                            result.push_str(INDENT);
                            result.push_str(&value.to_string());
                            result.push('\n');
                        }
                        None => break,
                    }
                }
                let remaining = bytes.len();
                if remaining != 0 {
                    result.push_str(&format!("{}[and {} more]\n", INDENT, remaining));
                }
                result.push('}');
            }
            Tag::String(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": '{}'", payload));
            }
            Tag::List(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {} entries\n{{\n", payload.len()));
                for tag in payload {
                    for line in tag.pretty().lines() {
                        result.push_str(INDENT);
                        result.push_str(&line);
                        result.push('\n');
                    }
                }
                result.push('}');
            }
            Tag::Compound(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {} entries\n{{\n", payload.len()));
                for tag in payload {
                    for line in tag.pretty().lines() {
                        result.push_str(INDENT);
                        result.push_str(&line);
                        result.push('\n');
                    }
                }
                result.push('}');
            }
            Tag::IntArray(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {} entries\n{{\n", payload.len()));
                let mut ints = payload.into_iter();
                for _ in 0..ABBREVIATE_ARRAY_SIZE {
                    match ints.next() {
                        Some(value) => {
                            result.push_str(INDENT);
                            result.push_str(&value.to_string());
                            result.push('\n');
                        }
                        None => break,
                    }
                }
                let remaining = ints.len();
                if remaining != 0 {
                    result.push_str(&format!("{}[and {} more]\n", INDENT, remaining));
                }
                result.push('}');
            }
            Tag::LongArray(name, payload) => {
                result.push_str(self.name());
                if let Some(value) = name {
                    result.push_str(&format!("(\"{}\")", value));
                };
                result.push_str(&format!(": {} entries\n{{\n", payload.len()));
                let mut longs = payload.into_iter();
                for _ in 0..ABBREVIATE_ARRAY_SIZE {
                    match longs.next() {
                        Some(value) => {
                            result.push_str(INDENT);
                            result.push_str(&value.to_string());
                            result.push('\n');
                        }
                        None => break,
                    }
                }
                let remaining = longs.len();
                if remaining != 0 {
                    result.push_str(&format!("{}[and {} more]\n", INDENT, remaining));
                }
                result.push('}');
            }
        }
        result
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
                match payload.first() {
                    Some(first) => buf.extend(first.bytes_id(byte_order)),
                    None => buf.extend(byte_order.bytes(0_i8)),
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

/// Quick way of creating a [`Tag::Byte`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, byte};
///
/// assert_eq!(
///     byte!(42),
///     Tag::Byte(None, 42)
/// );
///
/// assert_eq!(
///     byte!("The answer" => 42),
///     Tag::Byte(Some("The answer".to_string()), 42)
/// );
/// ```
#[macro_export]
macro_rules! byte {
    ($name:expr => $value:expr $(,)?) => {
        Tag::Byte(Some(String::from($name)), $value)
    };

    ($value:expr $(,)?) => {
        Tag::Byte(None, $value)
    };
}

/// Quick way of creating a [`Tag::Short`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, short};
///
/// assert_eq!(
///     short!(42),
///     Tag::Short(None, 42)
/// );
///
/// assert_eq!(
///     short!("The answer" => 42),
///     Tag::Short(Some("The answer".to_string()), 42)
/// );
/// ```
#[macro_export]
macro_rules! short {
    ($name:expr => $value:expr $(,)?) => {
        Tag::Short(Some(String::from($name)), $value)
    };

    ($value:expr $(,)?) => {
        Tag::Short(None, $value)
    };
}

/// Quick way of creating a [`Tag::Int`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, int};
///
/// assert_eq!(
///     int!(42),
///     Tag::Int(None, 42)
/// );
///
/// assert_eq!(
///     int!("The answer" => 42),
///     Tag::Int(Some("The answer".to_string()), 42)
/// );
/// ```
#[macro_export]
macro_rules! int {
    ($name:expr => $value:expr $(,)?) => {
        Tag::Int(Some(String::from($name)), $value)
    };

    ($value:expr $(,)?) => {
        Tag::Int(None, $value)
    };
}

/// Quick way of creating a [`Tag::Long`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, long};
///
/// assert_eq!(
///     long!(42),
///     Tag::Long(None, 42)
/// );
///
/// assert_eq!(
///     long!("The answer" => 42),
///     Tag::Long(Some("The answer".to_string()), 42)
/// );
/// ```
#[macro_export]
macro_rules! long {
    ($name:expr => $value:expr $(,)?) => {
        Tag::Long(Some(String::from($name)), $value)
    };

    ($value:expr $(,)?) => {
        Tag::Long(None, $value)
    };
}

/// Quick way of creating a [`Tag::Float`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, float};
///
/// assert_eq!(
///     float!(42.69),
///     Tag::Float(None, 42.69)
/// );
///
/// assert_eq!(
///     float!("The answer" => 42.69),
///     Tag::Float(Some("The answer".to_string()), 42.69)
/// );
/// ```
#[macro_export]
macro_rules! float {
    ($name:expr => $value:expr $(,)?) => {
        Tag::Float(Some(String::from($name)), $value)
    };

    ($value:expr $(,)?) => {
        Tag::Float(None, $value)
    };
}

/// Quick way of creating a [`Tag::Double`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, double};
///
/// assert_eq!(
///     double!(42.69),
///     Tag::Double(None, 42.69)
/// );
///
/// assert_eq!(
///     double!("The answer" => 42.69),
///     Tag::Double(Some("The answer".to_string()), 42.69)
/// );
/// ```
#[macro_export]
macro_rules! double {
    ($name:expr => $value:expr $(,)?) => {
        Tag::Double(Some(String::from($name)), $value)
    };

    ($value:expr $(,)?) => {
        Tag::Double(None, $value)
    };
}

/// Quick way of creating a [`Tag::ByteArray`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, byte_array};
///
/// assert_eq!(
///     byte_array!(1, 2, 3),
///     Tag::ByteArray(None, vec![1, 2, 3])
/// );
///
/// assert_eq!(
///     byte_array!("The answer" => 1, 2, 3),
///     Tag::ByteArray(Some("The answer".to_string()), vec![1, 2, 3])
/// );
/// ```
#[macro_export]
macro_rules! byte_array {
    ($name:expr => $($value:expr),* $(,)?) => {
        Tag::ByteArray(Some(String::from($name)), vec![$($value),*])
    };

    ($($value:expr),* $(,)?) => {
        Tag::ByteArray(None, vec![$($value),*])
    };
}

/// Quick way of creating a [`Tag::String`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, string};
///
/// assert_eq!(
///     string!("The name"),
///     Tag::String(None, "The name".to_string())
/// );
///
/// assert_eq!(
///     string!("The answer" => "The name"),
///     Tag::String(Some("The answer".to_string()), "The name".to_string())
/// );
/// ```
#[macro_export]
macro_rules! string {
    ($name:expr => $value:expr $(,)?) => {
        Tag::String(Some(String::from($name)), String::from($value))
    };

    ($value:expr $(,)?) => {
        Tag::String(None, String::from($value))
    };
}

/// Quick way of creating a [`Tag::List`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, byte, list};
///
/// assert_eq!(
///     list!(byte!(1), byte!(2), byte!(3)),
///     Tag::List(
///         None,
///         vec![
///             Tag::Byte(None, 1),
///             Tag::Byte(None, 2),
///             Tag::Byte(None, 3)
///         ]
///     )
/// );
///
/// assert_eq!(
///     list!("The answer" => byte!(1), byte!(2), byte!(3)),
///     Tag::List(
///         Some("The answer".to_string()),
///         vec![
///             Tag::Byte(None, 1),
///             Tag::Byte(None, 2),
///             Tag::Byte(None, 3),
///         ]
///     )
/// );
/// ```
///
/// This macro also has a shorthand for initializing a list with signed integers.
///
/// ```rust
/// use mcnbt::{Tag, list};
///
/// assert_eq!(
///     list!("The answer" => i8; 1, 2, 3),
///     Tag::List(
///         Some("The answer".to_string()),
///         vec![
///             Tag::Byte(None, 1),
///             Tag::Byte(None, 2),
///             Tag::Byte(None, 3),
///         ]
///     )
/// );
/// ```
///
/// You can use [`i8`], [`i16`], [`i32`] and [`i64`] to create the appropiate
/// [`Tag`]s.
#[macro_export]
macro_rules! list {
    ($name:expr => $($value:expr),* $(,)?) => {
        Tag::List(Some(String::from($name)), vec![$($value),*])
    };

    ($name:expr => $kind:ty; $($value:expr),* $(,)?) => {
        {
            macro_rules! eq_type {
                ($x:ty, $y:ty) => {{
                    use std::any::TypeId;
                    TypeId::of::<$x>() == TypeId::of::<$y>()
                }};
            }

            let mut data = vec![];
            $(
                data.push(
                    if eq_type!(i8, $kind) {
                        Tag::Byte(None, $value)
                    } else if eq_type!(i16, $kind) {
                        Tag::Short(None, $value)
                    } else if eq_type!(i32, $kind) {
                        Tag::Int(None, $value)
                    } else if eq_type!(i64, $kind) {
                        Tag::Long(None, $value)
                    /*
                    NOTE: not possible due to type validation at compile tim

                    } else if eq_type!(f32, $kind) {
                        Tag::Float(None, $value)
                    } else if eq_type!(f64, $kind) {
                        Tag::Double(None, $value)
                    } else if eq_type!(String, $kind) {
                        Tag::String(None, String::from($value))
                    */
                    } else {
                        unimplemented!("{} is an invalid type", std::any::type_name::<$kind>())
                    }
                );
            )*
            Tag::List(Some(String::from($name)), data)
        }
    };

    ($($value:expr),* $(,)?) => {
        Tag::List(None, vec![$($value),*])
    };
}

/// Quick way of creating a [`Tag::Compound`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, byte, compound, string};
///
/// assert_eq!(
///     compound!(
///         byte!("x" => 1),
///         byte!("y" => 2),
///         byte!("z" => 3),
///         string!("player_name" => "Steve"),
///     ),
///     Tag::Compound(
///         None,
///         vec![
///             Tag::Byte(Some("x".to_string()), 1),
///             Tag::Byte(Some("y".to_string()), 2),
///             Tag::Byte(Some("z".to_string()), 3),
///             Tag::String(Some("player_name".to_string()), "Steve".to_string()),
///         ]
///     )
/// );
///
/// assert_eq!(
///     compound!("The answer" =>
///         byte!("x" => 1),
///         byte!("y" => 2),
///         byte!("z" => 3),
///         string!("player_name" => "Steve"),
///     ),
///     Tag::Compound(
///         Some("The answer".to_string()),
///         vec![
///             Tag::Byte(Some("x".to_string()), 1),
///             Tag::Byte(Some("y".to_string()), 2),
///             Tag::Byte(Some("z".to_string()), 3),
///             Tag::String(Some("player_name".to_string()), "Steve".to_string()),
///         ]
///     )
/// );
/// ```
#[macro_export]
macro_rules! compound {
    ($name:expr => $($value:expr),* $(,)?) => {
        Tag::Compound(Some(String::from($name)), vec![$($value),*])
    };

    ($($value:expr),* $(,)?) => {
        Tag::Compound(None, vec![$($value),*])
    };
}

/// Quick way of creating a [`Tag::IntArray`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, int_array};
///
/// assert_eq!(
///     int_array!(1, 2, 3),
///     Tag::IntArray(None, vec![1, 2, 3])
/// );
///
/// assert_eq!(
///     int_array!("The answer" => 1, 2, 3),
///     Tag::IntArray(Some("The answer".to_string()), vec![1, 2, 3])
/// );
/// ```
#[macro_export]
macro_rules! int_array {
    ($name:expr => $($value:expr),* $(,)?) => {
        Tag::IntArray(Some(String::from($name)), vec![$($value),*])
    };

    ($($value:expr),* $(,)?) => {
        Tag::IntArray(None, vec![$($value),*])
    };
}

/// Quick way of creating a [`Tag::LongArray`].
///
/// # Example
///
/// ```rust
/// use mcnbt::{Tag, long_array};
///
/// assert_eq!(
///     long_array!(1, 2, 3),
///     Tag::LongArray(None, vec![1, 2, 3])
/// );
///
/// assert_eq!(
///     long_array!("The answer" => 1, 2, 3),
///     Tag::LongArray(Some("The answer".to_string()), vec![1, 2, 3])
/// );
/// ```
#[macro_export]
macro_rules! long_array {
    ($name:expr => $($value:expr),* $(,)?) => {
        Tag::LongArray(Some(String::from($name)), vec![$($value),*])
    };

    ($($value:expr),* $(,)?) => {
        Tag::LongArray(None, vec![$($value),*])
    };
}
