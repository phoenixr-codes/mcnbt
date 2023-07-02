use crate::byte_order::ByteOrder;

pub type Name<'a> = Option<&'a str>;

/// A tag is an individual part of the data tree. A tag consists of a name and
/// a payload. The name is absent when it is used within a [List].
///
/// The tag `TAG_End` is not available because it is handled by the program.
pub enum Tag<'a> {
    /// A signed integral type. Sometimes used for booleans.
    Byte(Name<'a>, i8),

    /// A signed integral type.
    Short(Name<'a>, i16),

    /// A signed intefral type.
    Int(Name<'a>, i32),

    /// A signed integral type.
    Long(Name<'a>, i64),

    /// A signed floating point type.
    Float(Name<'a>, f32),

    /// A signed floating point type.
    Double(Name<'a>, f64),

    /// An array of bytes.
    ByteArray(Name<'a>, &'a [i8]),

    /// A UTF-8 string.
    String(Name<'a>, &'a str),

    /// A list of tag payloads, without tag IDs or names.
    List(Name<'a>, &'a [Tag<'a>]),

    /// A list of fully formed tags, including their IDs, names, and payloads.
    Compound(Name<'a>, &'a [Tag<'a>]),

    /// An array of [Tag::Int]s.
    IntArray(Name<'a>, &'a [i32]),

    /// An array of [Tag::Long]s.
    LongArray(Name<'a>, &'a [i64]),
}

impl Tag<'_> {
    /// Returns a tag represented as bytes.
    pub fn as_bytes(&self, byte_order: ByteOrder) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.bytes_id(byte_order));
        buf.extend(
            self.bytes_name(byte_order)
                .expect("invalid structure: expected named tag"),
        );
        buf.extend(self.bytes_payload(byte_order));
        buf
    }

    /// Returns a vector of bytes with the length of the name and the name
    /// MUTF-8 encoded. This is an [std::result::Result::Err] when the name
    /// is absent.
    fn bytes_name(&self, byte_order: ByteOrder) -> Result<Vec<u8>, ()> {
        let mut buf = vec![];
        match *self {
            Tag::Byte(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::Short(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::Int(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::Long(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::Float(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::Double(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::ByteArray(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::String(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::List(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::Compound(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::IntArray(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
            Tag::LongArray(name, _) => {
                let n = name.ok_or(())?;
                let n = mutf8::encode(n).into_owned();
                let len: u16 = n.len().try_into().expect("name too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(n);
            }
        };
        Ok(buf)
    }

    /// Returns the ID of the tag as a vector of bytes.
    fn bytes_id(&self, byte_order: ByteOrder) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(match *self {
            Tag::Byte(_, _) => match byte_order {
                ByteOrder::BigEndian => 1_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 1_i8.to_le_bytes(),
            },
            Tag::Short(_, _) => match byte_order {
                ByteOrder::BigEndian => 2_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 2_i8.to_le_bytes(),
            },
            Tag::Int(_, _) => match byte_order {
                ByteOrder::BigEndian => 3_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 3_i8.to_le_bytes(),
            },
            Tag::Long(_, _) => match byte_order {
                ByteOrder::BigEndian => 4_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 4_i8.to_le_bytes(),
            },
            Tag::Float(_, _) => match byte_order {
                ByteOrder::BigEndian => 5_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 5_i8.to_le_bytes(),
            },
            Tag::Double(_, _) => match byte_order {
                ByteOrder::BigEndian => 6_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 6_i8.to_le_bytes(),
            },
            Tag::ByteArray(_, _) => match byte_order {
                ByteOrder::BigEndian => 7_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 7_i8.to_le_bytes(),
            },
            Tag::String(_, _) => match byte_order {
                ByteOrder::BigEndian => 8_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 8_i8.to_le_bytes(),
            },
            Tag::List(_, _) => match byte_order {
                ByteOrder::BigEndian => 9_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 9_i8.to_le_bytes(),
            },
            Tag::Compound(_, _) => match byte_order {
                ByteOrder::BigEndian => 10_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 10_i8.to_le_bytes(),
            },
            Tag::IntArray(_, _) => match byte_order {
                ByteOrder::BigEndian => 11_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 11_i8.to_le_bytes(),
            },
            Tag::LongArray(_, _) => match byte_order {
                ByteOrder::BigEndian => 12_i8.to_be_bytes(),
                ByteOrder::LittleEndian => 12_i8.to_le_bytes(),
            },
        });
        buf
    }

    /// Returns the payload of the tag as a vector of bytes.
    fn bytes_payload(&self, byte_order: ByteOrder) -> Vec<u8> {
        let mut buf = vec![];
        match *self {
            Tag::Byte(_, payload) => buf.extend(match byte_order {
                ByteOrder::BigEndian => payload.to_be_bytes(),
                ByteOrder::LittleEndian => payload.to_le_bytes(),
            }),

            Tag::Short(_, payload) => buf.extend(match byte_order {
                ByteOrder::BigEndian => payload.to_be_bytes(),
                ByteOrder::LittleEndian => payload.to_le_bytes(),
            }),

            Tag::Int(_, payload) => buf.extend(match byte_order {
                ByteOrder::BigEndian => payload.to_be_bytes(),
                ByteOrder::LittleEndian => payload.to_le_bytes(),
            }),

            Tag::Long(_, payload) => buf.extend(match byte_order {
                ByteOrder::BigEndian => payload.to_be_bytes(),
                ByteOrder::LittleEndian => payload.to_le_bytes(),
            }),

            Tag::Float(_, payload) => buf.extend(match byte_order {
                ByteOrder::BigEndian => payload.to_be_bytes(),
                ByteOrder::LittleEndian => payload.to_le_bytes(),
            }),

            Tag::Double(_, payload) => buf.extend(match byte_order {
                ByteOrder::BigEndian => payload.to_be_bytes(),
                ByteOrder::LittleEndian => payload.to_le_bytes(),
            }),

            Tag::ByteArray(_, payload) => {
                // length of array
                let len: i32 = payload.len().try_into().expect("byte array too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });

                // content of array
                for byte in payload {
                    buf.extend(match byte_order {
                        ByteOrder::BigEndian => byte.to_be_bytes(),
                        ByteOrder::LittleEndian => byte.to_le_bytes(),
                    });
                }
            }

            Tag::String(_, payload) => {
                let string = mutf8::encode(payload);
                let len: u16 = string.len().try_into().expect("string too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });
                buf.extend(string.into_owned());
            }

            Tag::List(_, payload) => {
                // tag ID
                if let Some(first) = payload.first() {
                    buf.extend(first.bytes_id(byte_order));
                }

                // length of list
                let len: i32 = payload.len().try_into().expect("list too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });

                // content of list
                for byte in payload {
                    buf.extend(byte.bytes_payload(byte_order));
                }
            }

            Tag::Compound(_, payload) => {
                for tag in payload {
                    buf.extend(tag.as_bytes(byte_order));
                }
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => 0_i8.to_be_bytes(),
                    ByteOrder::LittleEndian => 0_i8.to_le_bytes(),
                });
            }

            Tag::IntArray(_, payload) => {
                // length of array
                let len: i32 = payload.len().try_into().expect("int array too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });

                // content of array
                for int in payload {
                    buf.extend(match byte_order {
                        ByteOrder::BigEndian => int.to_be_bytes(),
                        ByteOrder::LittleEndian => int.to_le_bytes(),
                    });
                }
            }

            Tag::LongArray(_, payload) => {
                // length of array
                let len: i32 = payload.len().try_into().expect("long array too big");
                buf.extend(match byte_order {
                    ByteOrder::BigEndian => len.to_be_bytes(),
                    ByteOrder::LittleEndian => len.to_le_bytes(),
                });

                // content of array
                for long in payload {
                    buf.extend(match byte_order {
                        ByteOrder::BigEndian => long.to_be_bytes(),
                        ByteOrder::LittleEndian => long.to_le_bytes(),
                    });
                }
            }
        };
        buf
    }
}
