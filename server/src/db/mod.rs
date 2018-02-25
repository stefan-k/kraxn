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
use std::time::SystemTime;
use errors::*;
use models::{DataPoint, NewDataPoint};

pub mod schema;
pub mod models;

/// Establish a connection with the `PostgresSQL` database
pub fn establish_db_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    // this pattern of Ok(X?) is odd, but seems to be required to work with error-chain (?)
    Ok(PgConnection::establish(&database_url)?)
}

pub fn insert_dataset(plot_id: i32, x: f64, y: f64) -> Result<()> {
    use schema::datasets;
    let conn = establish_db_connection()?;

    let timestamp = SystemTime::now();

    let data_point = NewDataPoint {
        plot_id: plot_id,
        timestamp: timestamp,
        x: x,
        y: y,
    };
    diesel::insert_into(datasets::table)
        .values(&data_point)
        .execute(&conn)?;
    Ok(())
}

pub fn print_data(pid: i32) -> Result<()> {
    use schema::datasets::dsl::{datasets, plot_id};
    // use schema::datasets::dsl::*;

    let conn = establish_db_connection()?;

    let results = datasets.filter(plot_id.eq(pid)).load::<DataPoint>(&conn)?;

    for res in results {
        if let DataPoint {
            id,
            timestamp,
            plot_id: plid,
            x: Some(x),
            y: Some(y),
        } = res
        {
            println!("{}|{}|{:?}: [{}, {}]", id, plid, timestamp, x, y);
        }
    }

    Ok(())
}
