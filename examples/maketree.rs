extern crate mcnbt;

use mcnbt::{ByteOrder, Tag};

fn main() {
    /*
    let structure = Tag::Compound(
        Some(""),
        &[
            Tag::Int(Some("format_version"), 1),
            Tag::List(
                Some("size"),
                &[Tag::Int(None, 5), Tag::Int(None, 5), Tag::Int(None, 5)],
            ),
        ],
    );
    */
    let structure = Tag::Compound(Some(""), &[Tag::Int(Some("a"), 1)]);
    println!("{:#?}", structure.as_bytes(ByteOrder::LittleEndian));
}
