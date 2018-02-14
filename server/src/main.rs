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
use serde_json::Value;
use tokio_serde_json::ReadJson;
use errors::*;
use db::*;

/// Process a socket
fn process(socket: TcpStream, handle: &Handle) {
    // delimit frames using a length header
    let length_delimited = length_delimited::FramedRead::new(socket);

    // deserialize frames
    let deserialized =
        ReadJson::<_, Value>::new(length_delimited).map_err(|e| println!("Err: {:?}", e));

    println!("fu");
    // spawn a task that prints all received messages to STDOUT
    handle.spawn(deserialized.for_each(|msg| {
        println!("Got: {:?}", msg);
        create_post("bla", "blabla");
        create_post("fu", "blabla");
        publish_post(1);
        publish_post(2);
        print_posts().unwrap();
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

// use tokio::net::{TcpListener, TcpStream};
// use tokio::executor::current_thread;
// use tokio_io::io;
// use futures::{Future, Stream};
//
//
// fn process(socket: TcpStream) {
//     // TODO: accept connections, read data, store in db, reply
//     println!("fu");
//     let connection = io::write_all(socket, "hello world\n").then(|res| {
//         println!("wrote message; success={:?}", res.is_ok());
//         println!("res = {:?}", res);
//         current_thread::spawn(io::write_all(res.unwrap().0, "fu\n").then(|res| {
//             println!("fu");
//             Ok(())
//         }));
//         Ok(())
//     });
//     current_thread::spawn(connection);
// }
//
// fn run() -> Result<()> {
//     let addr = "127.0.0.1:6142".parse().unwrap();
//     let listener = TcpListener::bind(&addr).unwrap();
//
//     let server = listener
//         .incoming()
//         .for_each(move |socket| {
//             println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());
//
//             // spawn a new task that processes the socket
//             process(socket);
//             Ok(())
//         })
//         .map_err(|err| {
//             // all tasks must have an `Error` type of `()`. this forces error handling and helps avoid
//             // silcencing failures.
//             // log error
//             println!("accept error = {:?}", err);
//         });
//
//     current_thread::run(|_| {
//         // spawn server task
//         current_thread::spawn(server);
//         println!("server running on {}", addr);
//     });
//     Ok(())
// }
//
// fn main() {
//     if let Err(ref e) = run() {
//         println!("error: {}", e);
//     }
// }
