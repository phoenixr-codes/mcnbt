// TODO: improve/implement custom errors

use nom::bytes::complete::*;
use nom::combinator::eof;
use nom::error::{make_error, ErrorKind};
use nom::number::complete::*;
use nom::IResult;

use crate::byte_order::ByteOrder;
use crate::tag::Tag;

fn name(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], String> {
    let name_len: u16 = match byte_order {
        ByteOrder::BigEndian => {
            let res = be_u16(i)?;
            i = res.0;
            res.1
        }
        ByteOrder::LittleEndian => {
            let res = le_u16(i)?;
            i = res.0;
            res.1
        }
    };
    let name: String = {
        let res = take(name_len)(i)?;
        i = res.0;
        // TODO: provide error
        mutf8::decode(res.1)
            .map_err(|_| nom::Err::Failure(make_error(i, ErrorKind::Fail)))?
            .to_string()
    };
    Ok((i, name))
}

fn id(mut i: &[u8], allow_end_tag: bool) -> IResult<&[u8], i8> {
    let mut ids: Vec<u8> = vec![];
    if allow_end_tag {
        ids.push(0);
    }
    for i in 1..=12 {
        ids.push(i);
    }

    let tag_id;
    (i, tag_id) = nom::character::complete::one_of(ids.as_slice())(i)?;
    Ok((i, tag_id as i8))
}

fn byte(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = byte_payload(i, byte_order)?;

    Ok((i, Tag::Byte(Some(tag_name), tag_payload)))
}

fn byte_payload(i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], i8> {
    Ok(match byte_order {
        ByteOrder::BigEndian => be_i8(i)?,
        ByteOrder::LittleEndian => le_i8(i)?,
    })
}

fn short(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = short_payload(i, byte_order)?;

    Ok((i, Tag::Short(Some(tag_name), tag_payload)))
}

fn short_payload(i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], i16> {
    Ok(match byte_order {
        ByteOrder::BigEndian => be_i16(i)?,
        ByteOrder::LittleEndian => le_i16(i)?,
    })
}

fn int(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = int_payload(i, byte_order)?;

    Ok((i, Tag::Int(Some(tag_name), tag_payload)))
}

fn int_payload(i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], i32> {
    Ok(match byte_order {
        ByteOrder::BigEndian => be_i32(i)?,
        ByteOrder::LittleEndian => le_i32(i)?,
    })
}

fn long(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = long_payload(i, byte_order)?;

    Ok((i, Tag::Long(Some(tag_name), tag_payload)))
}

fn long_payload(i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], i64> {
    Ok(match byte_order {
        ByteOrder::BigEndian => be_i64(i)?,
        ByteOrder::LittleEndian => le_i64(i)?,
    })
}

fn float(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = float_payload(i, byte_order)?;

    Ok((i, Tag::Float(Some(tag_name), tag_payload)))
}

fn float_payload(i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], f32> {
    Ok(match byte_order {
        ByteOrder::BigEndian => be_f32(i)?,
        ByteOrder::LittleEndian => le_f32(i)?,
    })
}

fn double(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = double_payload(i, byte_order)?;

    Ok((i, Tag::Double(Some(tag_name), tag_payload)))
}

fn double_payload(i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], f64> {
    Ok(match byte_order {
        ByteOrder::BigEndian => be_f64(i)?,
        ByteOrder::LittleEndian => le_f64(i)?,
    })
}

fn byte_array(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = byte_array_payload(i, byte_order)?;

    Ok((i, Tag::ByteArray(Some(tag_name), tag_payload)))
}

fn byte_array_payload(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Vec<i8>> {
    let length;
    (i, length) = match byte_order {
        ByteOrder::BigEndian => be_i32(i)?,
        ByteOrder::LittleEndian => le_i32(i)?,
    };
    let mut bytes = vec![];
    for _ in 0..length {
        let byte;
        (i, byte) = match byte_order {
            ByteOrder::BigEndian => be_i8(i)?,
            ByteOrder::LittleEndian => le_i8(i)?,
        };
        bytes.push(byte);
    }
    Ok((i, bytes))
}

fn string(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = string_payload(i, byte_order)?;

    Ok((i, Tag::String(Some(tag_name), tag_payload)))
}

fn string_payload(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], String> {
    let length;
    (i, length) = match byte_order {
        ByteOrder::BigEndian => be_u16(i)?,
        ByteOrder::LittleEndian => le_u16(i)?,
    };
    let bytes;
    (i, bytes) = take_while_m_n(0, length.into(), |_| true)(i)?;
    let s = mutf8::decode(bytes).expect("TODO").to_string();
    Ok((i, s))
}

fn list(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = list_payload(i, byte_order)?;

    Ok((i, Tag::List(Some(tag_name), tag_payload)))
}

fn list_payload(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Vec<Tag>> {
    let tag_id;
    (i, tag_id) = id(i, false)?;

    let length;
    (i, length) = match byte_order {
        ByteOrder::BigEndian => be_i32(i)?,
        ByteOrder::LittleEndian => le_i32(i)?,
    };

    let mut tags: Vec<Tag> = vec![];
    for _ in 0..length {
        tags.push(match tag_id {
            1 => {
                let content;
                (i, content) = byte_payload(i, byte_order)?;
                Tag::Byte(None, content)
            }
            2 => {
                let content;
                (i, content) = short_payload(i, byte_order)?;
                Tag::Short(None, content)
            }
            3 => {
                let content;
                (i, content) = int_payload(i, byte_order)?;
                Tag::Int(None, content)
            }
            4 => {
                let content;
                (i, content) = long_payload(i, byte_order)?;
                Tag::Long(None, content)
            }
            5 => {
                let content;
                (i, content) = float_payload(i, byte_order)?;
                Tag::Float(None, content)
            }
            6 => {
                let content;
                (i, content) = double_payload(i, byte_order)?;
                Tag::Double(None, content)
            }
            7 => {
                let content;
                (i, content) = byte_array_payload(i, byte_order)?;
                Tag::ByteArray(None, content)
            }
            8 => {
                let content;
                (i, content) = string_payload(i, byte_order)?;
                Tag::String(None, content)
            }
            9 => {
                let content;
                (i, content) = list_payload(i, byte_order)?;
                Tag::List(None, content)
            }
            10 => {
                let content;
                (i, content) = compound_payload(i, byte_order)?;
                Tag::Compound(None, content)
            }
            11 => {
                let content;
                (i, content) = int_array_payload(i, byte_order)?;
                Tag::IntArray(None, content)
            }
            12 => {
                let content;
                (i, content) = long_array_payload(i, byte_order)?;
                Tag::LongArray(None, content)
            }
            _ => unreachable!(),
        });
    }

    Ok((i, tags))
}

fn compound(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = compound_payload(i, byte_order)?;

    Ok((i, Tag::Compound(Some(tag_name), tag_payload)))
}

fn compound_payload(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Vec<Tag>> {
    let mut tags: Vec<Tag> = vec![];
    let mut names: Vec<String> = vec![];
    loop {
        let tag_id;
        (i, tag_id) = id(i, true)?;

        if tag_id == 0 {
            break;
        }

        let tag_name;
        (i, tag_name) = name(i, byte_order)?;

        if names.contains(&tag_name) {
            return Err(nom::Err::Failure(make_error(i, ErrorKind::Fail)));
        }
        names.push(tag_name.clone());

        tags.push(match tag_id {
            1 => Tag::Byte(Some(tag_name), {
                let content;
                (i, content) = byte_payload(i, byte_order)?;
                content
            }),
            2 => Tag::Short(Some(tag_name), {
                let content;
                (i, content) = short_payload(i, byte_order)?;
                content
            }),
            3 => Tag::Int(Some(tag_name), {
                let content;
                (i, content) = int_payload(i, byte_order)?;
                content
            }),
            4 => Tag::Long(Some(tag_name), {
                let content;
                (i, content) = long_payload(i, byte_order)?;
                content
            }),
            5 => Tag::Float(Some(tag_name), {
                let content;
                (i, content) = float_payload(i, byte_order)?;
                content
            }),
            6 => Tag::Double(Some(tag_name), {
                let content;
                (i, content) = double_payload(i, byte_order)?;
                content
            }),
            7 => Tag::ByteArray(Some(tag_name), {
                let content;
                (i, content) = byte_array_payload(i, byte_order)?;
                content
            }),
            8 => Tag::String(Some(tag_name), {
                let content;
                (i, content) = string_payload(i, byte_order)?;
                content
            }),
            9 => Tag::List(Some(tag_name), {
                let content;
                (i, content) = list_payload(i, byte_order)?;
                content
            }),
            10 => Tag::Compound(Some(tag_name), {
                let content;
                (i, content) = compound_payload(i, byte_order)?;
                content
            }),
            11 => Tag::IntArray(Some(tag_name), {
                let content;
                (i, content) = int_array_payload(i, byte_order)?;
                content
            }),
            12 => Tag::LongArray(Some(tag_name), {
                let content;
                (i, content) = long_array_payload(i, byte_order)?;
                content
            }),
            _ => unreachable!(),
        });
    }

    Ok((i, tags))
}

fn int_array(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = int_array_payload(i, byte_order)?;

    Ok((i, Tag::IntArray(Some(tag_name), tag_payload)))
}

fn int_array_payload(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Vec<i32>> {
    let length;
    (i, length) = match byte_order {
        ByteOrder::BigEndian => be_i32(i)?,
        ByteOrder::LittleEndian => le_i32(i)?,
    };
    let mut bytes = vec![];
    for _ in 0..length {
        let byte;
        (i, byte) = match byte_order {
            ByteOrder::BigEndian => be_i32(i)?,
            ByteOrder::LittleEndian => le_i32(i)?,
        };
        bytes.push(byte);
    }
    Ok((i, bytes))
}

fn long_array(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_name;
    (i, tag_name) = name(i, byte_order)?;

    let tag_payload;
    (i, tag_payload) = long_array_payload(i, byte_order)?;

    Ok((i, Tag::LongArray(Some(tag_name), tag_payload)))
}

fn long_array_payload(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Vec<i64>> {
    let length;
    (i, length) = match byte_order {
        ByteOrder::BigEndian => be_i32(i)?,
        ByteOrder::LittleEndian => le_i32(i)?,
    };
    let mut bytes = vec![];
    for _ in 0..length {
        let byte;
        (i, byte) = match byte_order {
            ByteOrder::BigEndian => be_i64(i)?,
            ByteOrder::LittleEndian => le_i64(i)?,
        };
        bytes.push(byte);
    }
    Ok((i, bytes))
}

pub(crate) fn nbt(mut i: &[u8], byte_order: ByteOrder) -> IResult<&[u8], Tag> {
    let tag_id;
    (i, tag_id) = id(i, false)?;

    let tag;
    (i, tag) = match tag_id {
        1 => byte(i, byte_order)?,
        2 => short(i, byte_order)?,
        3 => int(i, byte_order)?,
        4 => long(i, byte_order)?,
        5 => float(i, byte_order)?,
        6 => double(i, byte_order)?,
        7 => byte_array(i, byte_order)?,
        8 => string(i, byte_order)?,
        9 => list(i, byte_order)?,
        10 => compound(i, byte_order)?,
        11 => int_array(i, byte_order)?,
        12 => long_array(i, byte_order)?,
        _ => unreachable!(),
    };

    eof(i)?;
    Ok((i, tag))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byte() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::Byte(Some("foo".to_string()), 42);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn short() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::Short(Some("foo".to_string()), 42);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn int() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::Int(Some("foo".to_string()), 42);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn long() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::Long(Some("foo".to_string()), 42);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn float() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::Float(Some("foo".to_string()), 42.69);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn double() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::Double(Some("foo".to_string()), 42.69);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn byte_array() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::ByteArray(Some("foo".to_string()), vec![42, 69, 127]);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn string() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::String(Some("foo".to_string()), String::from("Hello World"));
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn list() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::List(
                Some("foo".to_string()),
                vec![Tag::Byte(None, 1), Tag::Byte(None, 2), Tag::Byte(None, 3)],
            );
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn compound() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::Compound(
                Some("foo".to_string()),
                vec![
                    Tag::Byte(Some("x".to_string()), 1),
                    Tag::Byte(Some("y".to_string()), 2),
                    Tag::Byte(Some("z".to_string()), 3),
                ],
            );
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn int_array() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::IntArray(Some("foo".to_string()), vec![42, 69, 420]);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }

    #[test]
    fn long_array() {
        for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
            let data = Tag::LongArray(Some("foo".to_string()), vec![42, 69, 420]);
            assert_eq!(
                nbt(data.to_bytes(byte_order).unwrap().as_slice(), byte_order).unwrap(),
                (b"".as_slice(), data)
            );
        }
    }
}
