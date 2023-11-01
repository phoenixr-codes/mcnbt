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
    let structure = mcnbt::nbt![
        mcnbt::list!("foo" => mcnbt::int!(1), mcnbt::int!(2), mcnbt::int!(3)),
        mcnbt::compound!("bar" =>
            mcnbt::byte!("x" => 1),
            mcnbt::byte!("y" => 2),
            mcnbt::byte!("z" => 3),
        ),
        mcnbt::string!("hello" => "world"),
        mcnbt::int_array!("baz" => 10, 20, 30),
    ];

    println!("Bytes\n{:#?}", structure.to_bytes(ByteOrder::LittleEndian));
    println!("Pretty\n{:#?}", structure);
}
