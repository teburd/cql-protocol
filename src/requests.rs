use std::io::{self, Write};
use std::collections::HashMap;
use byteorder::{WriteBytesExt, BigEndian};

use Serialize;
use frame::{self, FRAME_SIZE, Frame, Version, OpCode};
use types::{CqlVersion, Compression, ToCql};

pub enum Request {
    Options(Options),
    Startup(Startup),
    Query(Query),
}

impl Serialize for Request {
    fn serialize(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match *self {
            Request::Options(ref options) => options.serialize(buf),
            Request::Startup(ref startup) => startup.serialize(buf),
            Request::Query(ref query) => query.serialize(buf),
        }
    }
}


// Options message to determine what Options the server supports for each of the Startup attributes
pub struct Options;

impl Serialize for Options {
    fn serialize(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        Ok(0)
    }
}

// Startup message to set both the CQL version and compression settings
//based on the available options given by the Options results
pub struct Startup {
    pub cql_version: CqlVersion,
    pub compression: Compression,
}

impl Serialize for Startup {
    fn serialize(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        try!(buf.write_i16::<BigEndian>(2));
        let mut written = 2;
        written += try!(write_str("CQL_VERSION", buf));
        written += try!(write_str(self.cql_version.to_str(), buf));
        written += try!(write_str("COMPRESSION", buf));
        written += try!(write_str(self.compression.to_str(), buf));
        Ok(written)
    }
}



pub fn write_str(s: &str, buf: &mut Vec<u8>) -> io::Result<usize> {
    try!(buf.write_i16::<BigEndian>(s.len() as i16));
    let mut written = 2;
    written += try!(buf.write(s.as_bytes()));
    Ok(written)
}

pub enum QueryParams {
    None,
    Positional(Vec<Box<ToCql>>),
    Named(HashMap<String, Box<ToCql>>),
}

mod query_flags {
    bitflags! {
        pub flags Flags: u8 {
            const VALUES = 0x01,
            const SKIP_METADATA = 0x02,
            const PAGE_SIZE = 0x04,
            const PAGING_STATE = 0x08,
            const SERIAL_CONSISTENCY = 0x10,
            const DEFAULT_TIMESTAMP = 0x20,
            const NAMED_PARAMS = 0x40,
        }
    }
}

pub enum Consistency {
    Any = 0x0000,
    One = 0x0001,
    Two = 0x0002,
    Three = 0x0003,
    Quorum = 0x0004,
    All = 0x0005,
    LocalQuorum = 0x0006,
    EachQuorum = 0x0007,
    Serial = 0x0008,
    LocalSerial = 0x0009,
    LocalOne = 0x000A,
}

pub struct Query {
    pub query: String,
    pub consistency: Consistency,
    pub params: QueryParams,
    pub page_size: Option<usize>,
    pub paging_state: Option<Vec<u8>>,
    pub skip_metadata: bool,
    pub serial_consistency: bool,
}

impl Serialize for Query {
    fn serialize(&self, buf: &mut Vec<u8>)  -> io::Result<usize> {
        Ok(0)
    }
}

#[test]
fn test_write_str() {
    let teststr = "TEST";
    let mut buf = Vec::new();
    assert!(write_str(&teststr, &mut buf).unwrap() == 6);
}
