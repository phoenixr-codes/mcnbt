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


## Using the CLI

```console
cargo install mcnbt
```

```text
Usage: nbt [OPTIONS] <path>

Arguments:
  <path>  The path to the NBT file

Options:
  -L, --little-endian  Use little endian byte order
  -h, --help           Print help
  -V, --version        Print version
```


## Using the Web

The `web` directory conatains a web interface which uses `mcnbt` in the back-end.
See it in action [here](https://phoenixr-codes.github.io/mcnbt/).


## Using `serde`

```console
cargo add -F serde mcnbt
```

```rust
use mcnbt::Tag;
use serde_json::Value;

let data = Tag::Compound(
    Some("".to_string()),
    vec![
        Tag::String(
            Some("foo".to_string()),
            "Hello World".to_string()
        ),
        Tag::List(
            Some("bar".to_string()),
            vec![
                Tag::Byte(None, 1),
                Tag::Byte(None, 2),
                Tag::Byte(None, 3),
            ]
        )
    ]
);

assert_eq!(
    serde_json::to_string(&data).unwrap(),
    serde_json::to_string(
        &serde_json::json!({
            "type": "compound",
            "name": "",
            "payload": [
                {
                    "type": "string",
                    "name": "foo",
                    "payload": "Hello World"
                },
                {
                    "type": "list",
                    "name": "bar",
                    "payload": [
                        {
                            "type": "byte",
                            "name": null,
                            "payload": 1
                        },
                        {
                            "type": "byte",
                            "name": null,
                            "payload": 2
                        },
                        {
                            "type": "byte",
                            "name": null,
                            "payload": 3
                        }
                    ]
                }
            ]
        })
    ).unwrap()
);
```


## Resources

Here are some websites explaining the NBT file format that have been used for the
development of this library.

- <https://wiki.vg/NBT>
- <https://minecraft.wiki/w/NBT_format#Binary_format>
- <https://wiki.bedrock.dev/nbt/nbt-in-depth.html>


## Contributing

### Running Tests

```console
cargo test --all-features
#          ^^^^^^^^^^^^^^ important
```
