// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Database interaction

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use errors::*;

pub mod schema;
pub mod models;

/// Establish a connection with the PostgresSQL database
pub fn establish_db_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    // this pattern of Ok(X?) is odd, but seems to be required to work with error-chain (?)
    Ok(PgConnection::establish(&database_url)?)
}

pub fn print_posts() -> Result<()> {
    use schema::posts::dsl::*;
    use models::*;

    let connection = establish_db_connection()?;
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts.");
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
    Ok(())
}
