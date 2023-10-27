#![doc = include_str!("../README.md")]

pub mod byte_order;
pub mod errors;
#[cfg(feature = "read")]
pub(crate) mod parser;
pub mod tag;

pub use byte_order::ByteOrder;
pub use tag::Tag;
