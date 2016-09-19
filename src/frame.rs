use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

use {Serialize, ParseResult, Parse};

pub const FRAME_SIZE: usize = 9;

#[derive(Debug, Copy, Clone)]
pub enum Version {
    RequestV3 = 0x03,
    ResponseV3 = 0x83,
}

impl Version {
    pub fn from_byte(byte: u8) -> io::Result<Version> {
        match byte {
            0x03 => Ok(Version::RequestV3),
            0x83 => Ok(Version::ResponseV3),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput,
            format!("unknown version byte `{}`", byte)))
        }
    }
}

pub mod flags {
    bitflags! {
        pub flags Flags: u8 {
            const NONE = 0x00,
            const COMPRESSED = 0x01,
            const TRACING = 0x02,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Error = 0x00,
    Startup = 0x01,
    Ready = 0x02,
    Authenticate = 0x03,
    Options = 0x05,
    Supported = 0x06,
    Query = 0x07,
    Result = 0x08,
    Prepare = 0x09,
    Execute = 0x0A,
    Register = 0x0B,
    Event = 0x0C,
    Batch = 0x0D,
    AuthChallenge = 0x0E,
    AuthResponse = 0x0F,
    AuthSuccess = 0x10,
}

impl OpCode {
    pub fn from_byte(byte: u8) -> io::Result<OpCode> {
        match byte {
            0x00 => Ok(OpCode::Error),
            0x01 => Ok(OpCode::Startup),
            0x02 => Ok(OpCode::Ready),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput,
                                    format!("unknown opcode byte `{}`", byte)))
        }
    }
}

// Frame contains the protocol message framing information
// such as protocol version, flags, multiplexed stream id, opcode,
// and length of the body.
pub struct Frame {
    pub version: Version,
    pub flags: flags::Flags,
    pub stream: i16,
    pub opcode: OpCode,
    pub length: i32,
}


impl Serialize for Frame {
    fn serialize(&self, buf: &mut Vec<u8>) -> io::Result<usize>{
        let vsn = self.version as u8;
        buf.push(vsn);
        buf.push(self.flags.bits());
        try!(buf.write_i16::<BigEndian>(self.stream));
        let opcode = self.opcode as u8;
        buf.push(opcode);
        try!(buf.write_i32::<BigEndian>(self.length));
        Ok(FRAME_SIZE)
    }
}

impl Parse<Frame> for Frame {
    fn parse(buf: &[u8]) -> io::Result<ParseResult<Frame>> {
        if buf.len() < FRAME_SIZE {
            Ok(ParseResult::Incomplete{ required_size: None })
        } else {
            let mut buf = &buf[0..FRAME_SIZE];
            let version = try!(Version::from_byte(try!(buf.read_u8())));
            let flags = flags::Flags::from_bits_truncate(try!(buf.read_u8()));
            let stream = try!(buf.read_i16::<BigEndian>());
            let opcode = try!(OpCode::from_byte(try!(buf.read_u8())));
            let length = try!(buf.read_i32::<BigEndian>());
            Ok(ParseResult::Complete{
                message: Frame {
                    version: version,
                    flags: flags,
                    stream: stream,
                    opcode: opcode,
                    length: length,
                },
                consumed: FRAME_SIZE,
            })
        }
    }
}
