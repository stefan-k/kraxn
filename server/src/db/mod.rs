// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Database interaction

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use errors::*;
use models::{NewPost, Post};

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

pub fn create_post<'a>(title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let conn = establish_db_connection().unwrap();

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&conn)
        .expect("error saving new post")
}

pub fn publish_post(id: i32) {
    use schema::posts::dsl::{posts, published};
    let conn = establish_db_connection().unwrap();
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&conn)
        .expect(&format!("unable to find post {}", id));
    println!("published post {}", post.title);
}

pub fn delete_post<'a>(target: &'a str) {
    use schema::posts::dsl::*;

    let pattern = format!("%{}%", target);

    let conn = establish_db_connection().unwrap();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&conn)
        .expect("error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
