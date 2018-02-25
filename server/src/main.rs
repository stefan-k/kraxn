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
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_serde_json;

/// Errors
mod errors;

/// Database
mod db;

use futures::Stream;
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_io::codec::length_delimited;
// use serde_json::Value;
use tokio_serde_json::ReadJson;
use errors::*;
use db::*;

/// TODO: use the one from models
#[derive(Serialize, Deserialize, Debug)]
pub struct DataP {
    /// plot id
    pub plot_id: i32,
    /// data
    pub data: Vec<f64>,
}

/// Process a socket
fn process(socket: TcpStream, handle: &Handle) {
    // delimit frames using a length header
    let length_delimited = length_delimited::FramedRead::new(socket);

    // deserialize frames
    let deserialized =
        ReadJson::<_, DataP>::new(length_delimited).map_err(|e| println!("Err: {:?}", e));

    // spawn a task that prints all received messages to STDOUT
    handle.spawn(deserialized.for_each(|msg: DataP| {
        println!("Got: {:?}", msg);
        insert_dataset(msg.plot_id, msg.data[0], msg.data[1]).unwrap();
        print_data(1).unwrap();
        Ok(())
    }));
}

fn run() -> Result<()> {
    let mut core = Core::new()?;
    let handle = core.handle();

    // bind a server socket
    let listener = TcpListener::bind(&"127.0.0.1:17653".parse()?, &handle)?;

    println!("Listening on {:?}", listener.local_addr());

    core.run(listener.incoming().for_each(|(socket, _)| {
        process(socket, &handle);
        Ok(())
    })).unwrap();
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
    }
}
