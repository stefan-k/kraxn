// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Database models

// use super::schema::posts;
use super::schema::datasets;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct DataPoint {
    pub id: i32,
    pub timestamp: SystemTime,
    pub plot_id: i32,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Insertable)]
#[table_name = "datasets"]
pub struct NewDataPoint {
    pub plot_id: i32,
    pub timestamp: SystemTime,
    pub x: f64,
    pub y: f64,
}

// #[derive(Queryable)]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }
//
// #[derive(Insertable)]
// #[table_name = "posts"]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }
