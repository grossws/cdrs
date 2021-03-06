//! **cdrs** is a native Cassandra DB client written in Rust.
//! It's under a hard development as of now.
extern crate snap;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate lz4_compress;
extern crate uuid;
#[cfg(feature = "ssl")]
extern crate openssl;
extern crate r2d2;

use std::io::Cursor;

pub mod frame;
pub mod types;

pub mod authenticators;
pub mod client;
pub mod compression;
pub mod connection_manager;
pub mod consistency;
pub mod error;
pub mod events;
pub mod query;
pub mod transport;


/// `IntoBytes` should be used to convert a structure into array of bytes.
pub trait IntoBytes {
    /// It should convert a struct into an array of bytes.
    fn into_cbytes(&self) -> Vec<u8>;
}

/// `FromBytes` should be used to parse an array of bytes into a structure.
pub trait FromBytes {
    /// It gets and array of bytes and should return an implementor struct.
    fn from_bytes(Vec<u8>) -> Self;
}

/// `AsBytes` should be used to convert a value into a single byte.
pub trait AsByte {
    /// It should represent a struct as a single byte.
    fn as_byte(&self) -> u8;
}

/// `FromSingleByte` should be used to convert a single byte into a value.
/// It is opposite to `AsByte`.
pub trait FromSingleByte {
    /// It should convert a single byte into an implementor struct.
    fn from_byte(u8) -> Self;
}

/// `FromCursor` should be used to get parsed structure from an `io:Cursor`
/// wich bound to an array of bytes.
pub trait FromCursor {
    /// It should return an implementor from an `io::Cursor` over an array of bytes.
    fn from_cursor(&mut Cursor<Vec<u8>>) -> Self;
}
