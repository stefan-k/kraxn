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
extern crate tokio;
extern crate tokio_io;

use tokio::net::{TcpListener, TcpStream};
use tokio::executor::current_thread;
use tokio_io::io;
use futures::{Future, Stream};

error_chain!{}

fn process(socket: TcpStream) {
    // TODO: accept connections, read data, store in db, reply
    println!("fu");
    let connection = io::write_all(socket, "hello world\n").then(|res| {
        println!("wrote message; success={:?}", res.is_ok());
        println!("res = {:?}", res);
        current_thread::spawn(io::write_all(res.unwrap().0, "fu\n").then(|res| {
            println!("fu");
            Ok(())
        }));
        Ok(())
    });
    current_thread::spawn(connection);
}

fn run() -> Result<()> {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(move |socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            // spawn a new task that processes the socket
            process(socket);
            Ok(())
        })
        .map_err(|err| {
            // all tasks must have an `Error` type of `()`. this forces error handling and helps avoid
            // silcencing failures.
            // log error
            println!("accept error = {:?}", err);
        });

    current_thread::run(|_| {
        // spawn server task
        current_thread::spawn(server);
        println!("server running on {}", addr);
    });
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
    }
}
