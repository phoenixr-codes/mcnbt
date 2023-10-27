use mcnbt::{ByteOrder, Tag};

fn main() {
    // using structs
    let structure = Tag::Compound(
        Some("".to_string()),
        vec![
            Tag::Int(Some("format_version".to_string()), 1),
            Tag::List(
                Some("size".to_string()),
                vec![Tag::Int(None, 5), Tag::Int(None, 5), Tag::Int(None, 5)],
            ),
        ],
    );

    println!("Bytes\n{:#?}", structure.to_bytes(ByteOrder::LittleEndian));
    println!("Pretty\n{:#?}", structure);

    // using macros

    // TODO
}
