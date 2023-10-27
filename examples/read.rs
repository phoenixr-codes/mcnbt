use mcnbt::{errors, ByteOrder, Tag};

fn main() -> Result<(), errors::Error> {
    let tag = Tag::from_bytes(
        include_bytes!("hello_world.nbt"), // file to read
        ByteOrder::BigEndian,              // Java Edition uses big endian byte order
    )?;
    println!("{:#?}", tag);

    let tag = Tag::from_bytes(include_bytes!("bigtest.nbt"), ByteOrder::BigEndian)?;
    println!("{:#?}", tag);

    Ok(())
}
