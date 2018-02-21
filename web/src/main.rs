// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;
use rocket::response::content;
use rocket::response::NamedFile;

#[derive(FromForm)]
struct Message {
    message: String,
}

#[post("/submit", data = "<message>")]
fn submit(mut cookies: Cookies, message: Form<Message>) -> Redirect {
    cookies.add_private(Cookie::new("message", message.into_inner().message));
    Redirect::to("/")
}

#[get("/")]
fn index(mut cookies: Cookies) -> Template {
    let cookie = cookies.get_private("message");
    let mut context = HashMap::new();
    if let Some(ref cookie) = cookie {
        context.insert("message", cookie.value());
    }

    Template::render("index", &context)
}

#[get("/d3")]
fn d3() -> Template {
    let mut context = HashMap::new();
    context.insert("blabla", "bla");
    Template::render("d3", &context)
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, World"
}

#[get("/js/<file..>")]
fn js_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("js/").join(file)).ok()
}

#[error(404)]
fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!(
        "<p>Sorry, but '{}' is not a valid path!</p>
         <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
        req.uri()
    ))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![submit, index, hello, d3, js_files])
        .catch(errors![not_found])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, World".into()));
    }
}
