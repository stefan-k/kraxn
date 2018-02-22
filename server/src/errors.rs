// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Errors

use std;
use diesel;

error_chain!{
    foreign_links {
        IoError(std::io::Error);
        AddrParseError(std::net::AddrParseError);
        VarError(std::env::VarError);
        ConnectionError(diesel::ConnectionError);
        DieselError(diesel::result::Error);
    }
}