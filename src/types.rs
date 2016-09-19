use std::io;

pub enum CqlVersion {
    V_3_0_0,
}

impl CqlVersion {
    pub fn to_str(&self) -> &'static str {
    match *self {
        CqlVersion::V_3_0_0 => "3.0.0",
}
    }
}

pub enum Compression {
    LZ4,
    Snappy,
}

impl Compression {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Compression::LZ4 => "lz4",
            Compression::Snappy => "snappy",
        }
    }
}

pub enum CqlValue {
    Int(i32),
}

pub struct CqlOption {
    id: i16,
    value: CqlValue,
}


pub enum CqlTypeCode {
    Custom = 0x0000,
    Ascii = 0x0001,
    Bigint = 0x0002,
    Blob = 0x0003,
    Boolean = 0x0004,
    Counter = 0x0005,
    Decimal = 0x0006,
    Double = 0x0007,
    Float = 0x0008,
    Int = 0x0009,
    Timestamp = 0x000B,
    Uuid = 0x000C,
    Varchar = 0x000D,
    Varint = 0x000E,
    TimeUuid = 0x000F,
    Inet = 0x0010,
    List = 0x0020,
    Map = 0x0021,
    Set = 0x0022,
    Udt = 0x0030,
    Tuple = 0x0031,
}

pub enum CqlType {
    Custom(String),
    Ascii,
    Bigint,
    Blob,
    Boolean,
    Counter,
    Decimal,
    Double,
    Float,
    Int,
    Timestamp,
    Uuid,
    Varchar,
    Varint,
    Timeuuid,
    Inet,
    //List(CqlOption),
    //Map(CqlOption, CqlOption),
    //Set(CqlOption),
    //UDT(CqlUDT),
    //Tuple(CqlTuple),
}

pub trait ToCql {
    fn write(&self, buf: &mut Vec<u8>) -> io::Result<()>;
}
