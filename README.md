# CQL Protocol

A low level implementation of the CQL binary protocol for Rust.

Provides methods for serialization and parsing of request and response
messages CQL implementations use to communicate.

In general the way to send a message to the server would be something like...

``` rust
let startup = cql::requests::Startup {
    cqlversion: cql::types::CqlVersion::V_3_0_0,
    compression: cql::types::Compression::LZ4,
}

let mut body_buf = Vec::new();
let length = startup.serialize(&mut buf).unwrap();
let frame = cql::frame::Frame {
    version: cql::frame::Version::RequestV3,
    flags: cql::frame::flags::None,
    stream: 0,
    opcode: cql::frame::OpCode::Startup,
    length: length
};
let mut header_buf = Vec::new();
frame.serialize(&mut header_buf);
socket.write(header_buf);
socket.write(body_buf);
```

Receiving a message works similiarly except you need to build up a buffer until
parsing works in whole. The length required is given. Each frame header is 9
bytes and must be decoded first. The length of the message is given in the frame
header.

``` rust
match cql::response::Response::parse(input_buf) {
    Err(err) => panic!(err), // do something smart here
    Ok(result) => {
        // result is a ParseResult and must be used to determine what is needed
        match result {
            Complete { message: message, consumed: consumed } => {
                println!("got message {:?}, consumed {}", message, consumed);
            },
            Incomplete { required_size: Some(required) } => {
                println!("buffer does not contain enough of the message, parsing knows it needs {} bytes", n);
            },
            Incomplete { required_size: None } => {
                println!("buffer needs unknown more bytes, but keep feeding!");
            }
        }
    }
}
```



