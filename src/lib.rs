//! Low Level CQL protocol APIs.
//!
//! This crate implements the low level components of the CQL v3 binary protocol
//! use by Apache Cassandra and ScyllaDB, including message and value serialization
//! and de-serialization. It is designed to be used as a building block by higher
//! level APIs such as `tokio-cql`, and should not typically be used directly.
//#![warn(missing_docs)]
extern crate byteorder;

#[macro_use]
extern crate bitflags;

pub mod frame;
pub mod requests;
pub mod responses;
pub mod types;

use std::io;

/// Serialize a message type into a buffer for sending out
pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>) -> io::Result<usize>;
}

/// Parse a message type from a buffer, return informs parser state
pub enum ParseResult<T> {
    Complete {
        message: T,
        consumed: usize,
    },
    Incomplete {
        required_size: Option<usize>,
    },
}

/// Parse trait for types that may be parsed from a given buffer
pub trait Parse<T> {
    fn parse(buf: &[u8]) -> io::Result<ParseResult<T>>;
}

pub use requests::Request;
pub use responses::Response;
