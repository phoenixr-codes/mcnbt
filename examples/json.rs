use mcnbt::{list, nbt, string, Tag};

fn main() {
    let data = nbt![list!("Hellou" => i8; 1, 2, 3), string!("World" => "!!!")];
    let json = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", json);

    let data = Tag::from_bytes(include_bytes!("bigtest.nbt"), mcnbt::ByteOrder::BigEndian).unwrap();
    let json = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", json);
}
