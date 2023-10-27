# mcnbt

Read and write NBT files.

[![docs.rs](https://img.shields.io/docsrs/mcnbt/latest)](https://docs.rs/mcnbt/latest/mcnbt/)
[![Crates.io](https://img.shields.io/crates/v/mcnbt)](https://crates.io/crates/mcnbt)


## Installation

```console
cargo add mcnbt
```


## Examples

Reading NBT data.

```rust
use mcnbt::{ByteOrder, Tag};

let tag = Tag::from_bytes(
    include_bytes!("../examples/hello_world.nbt"), // file to read
    ByteOrder::BigEndian // Java Edition uses big endian byte order
);
println!("{:#?}", tag);
```

Writing NBT data.

```rust
use mcnbt::{ByteOrder, Tag};

let tag = mcnbt::nbt![
    Tag::Int(Some("foo".to_string()), 42),
    Tag::List(Some("bar".to_string()), vec![
        Tag::String(None, "Hello".to_string()),
        Tag::String(None, "World".to_string()),
    ]),
    Tag::ByteArray(Some("baz".to_string()), vec![
        -8,
        -6,
        -4,
        -2,
        0,
        2,
        4,
        6,
        8,
    ]),
];
println!("{:#?}", tag.to_bytes(ByteOrder::LittleEndian));
```


## Resources

- https://wiki.vg/NBT
- https://minecraft.wiki/w/NBT_format#Binary_format
- https://wiki.bedrock.dev/nbt/nbt-in-depth.html

