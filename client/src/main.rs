// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO Documentation

#![recursion_limit = "1024"]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(missing_docs)]
#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_serde_json;

/// `std::io::Error`
/// `std::net::AddrParseError`
error_chain!{
    foreign_links {
        IoError(std::io::Error);
        AddrParseError(std::net::AddrParseError);
    }
}

use futures::{Future, Sink};
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

// use length delmited frames
use tokio_io::codec::length_delimited;

use tokio_serde_json::WriteJson;

fn run() -> Result<()> {
    let mut core = Core::new()?;
    let handle = core.handle();

    // bin a server socket
    let socket = TcpStream::connect(&"127.0.0.1:17653".parse()?, &handle);

    core.run(socket.and_then(|socket| {
        // delimit frames using a length header
        let length_delimited = length_delimited::FramedWrite::new(socket);

        // serialize frames with JSON
        let serialized = WriteJson::new(length_delimited);

        // Send the value
        serialized.send(json!({
            "name": "fu",
            "age": 42,
            "phones": [
                "1234",
                "5678"
            ]
        }))
    })).unwrap();
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
    }
}
