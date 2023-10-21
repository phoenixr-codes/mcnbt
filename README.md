# mcnbt

Read and write NBT files.[^1]

[![docs.rs](https://img.shields.io/docsrs/mcnbt/latest)](https://docs.rs/mcnbt/latest/mcnbt/)
[![Crates.io](https://img.shields.io/crates/v/mcnbt)](https://crates.io/crates/mcnbt)

[^1]: Reading not yet supported.

## Installation

```console
cargo add mcnbt
```


## Examples
<!--
Reading NBT data.

```rust
use mcnbt::{ByteOrder, Tag};

let tag = Tag::from_bytes(
    include_bytes!("house.mcstructure"),  // file to read
    ByteOrder::LittleEndian               // Bedrock Edition uses little endian byte order
);
println!(tag.pretty());
```
-->

Writing NBT data.

```rust
use mcnbt::{ByteOrder, Tag};

let tag = Tag::Compound(
    Some(""),  // Most outer tag is a compound with an empty name
    &[
        Tag::Int(Some("foo"), 42),
        Tag::List(Some("bar"), &[
            Tag::String(None, "Hello"),
            Tag::String(None, "World"),
        ]),
        Tag::ByteArray(Some("baz"), &[
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
    ],
);
println!("{:#?}", tag.as_bytes(ByteOrder::LittleEndian));
```
