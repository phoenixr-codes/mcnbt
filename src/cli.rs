use clap::{Arg, Command};
use mcnbt::byte_order::ByteOrder;
use mcnbt::tag::Tag;

use std::fs;
use std::path::PathBuf;

fn cmd() -> Command {
    Command::new("nbt")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jonas da Silva")
        .args([
            Arg::new("little_endian")
                .short('L')
                .long("little-endian")
                .help("Use little endian byte order")
                .num_args(0),
            Arg::new("path")
                .help("The path to the NBT file")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
            Arg::new("truncate")
                .short('t')
                .long("truncate")
                .help("Maximum amount of array items to display or 0 to display all items")
                .value_parser(clap::value_parser!(u64))
                .default_value("50"),
        ])
}

fn main() {
    let matches = cmd().get_matches();
    let byte_order = if matches.get_flag("little_endian") {
        ByteOrder::LittleEndian
    } else {
        ByteOrder::BigEndian
    };
    let truncate: &u64 = matches.get_one("truncate").unwrap();
    let path: &PathBuf = matches.get_one("path").unwrap();
    let content = fs::read(path).expect(&format!("file '{}' does not exist", path.display()));

    let data = Tag::from_bytes(content.as_slice(), byte_order).unwrap();
    println!("{}", data.pretty_truncated(*truncate));
}
