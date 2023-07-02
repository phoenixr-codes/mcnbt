#![doc = include_str!("../README.md")]

pub mod byte_order;
pub mod tag;

pub use byte_order::ByteOrder;
pub use tag::Tag;

/// The version of the NBT format this crate uses.
pub const FORMAT_VERSION: usize = 19133;
