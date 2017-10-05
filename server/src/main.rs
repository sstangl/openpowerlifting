#![feature(plugin)]
#![plugin(rocket_codegen)]

#![recursion_limit="256"] // For Diesel.
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate r2d2_diesel;
extern crate r2d2;

use diesel::prelude::*;

extern crate rocket;
use rocket::http::Cookies;

use std::error::Error;
use std::path::{Path, PathBuf};

pub mod schema;
use schema::Entry;
use schema::Meet;
use schema::DbConn;


#[get("/")]
fn index(cookies: Cookies) -> &'static str {
    "Hello, world!"
}


// TODO: Don't use Box<Error> -- use a custom error type?
#[get("/meet/<meetpath..>")]
fn meet_handler(meetpath: PathBuf, conn: DbConn) -> Result<String, Box<Error>> {
    let meetpath_str = try!(meetpath.to_str().ok_or(
        std::io::Error::new(std::io::ErrorKind::Other, "Malformed string.")));

    let meet_result: QueryResult<Meet> =
        schema::meets::table.filter(schema::meets::MeetPath.eq(meetpath_str))
        .first::<Meet>(&*conn);

    if meet_result.is_err() {
        return Ok(String::from("Meet not found."));
    }

    let meet = meet_result.unwrap();
    let meetid: i32 = meet.id.unwrap();

    let entries = schema::entries::table.filter(schema::entries::MeetID.eq(meetid))
                  .load::<Entry>(&*conn)
                  .expect("Error loading entries.");

    let mut display = String::new();

    for entry in entries {
        let name = entry.name.unwrap();
        display.push_str(name.as_str());
        display.push_str("\n");
    }

    Ok(display)
}


fn main() {
    rocket::ignite()
        .manage(schema::init_pool())
        .mount("/", routes![index])
        .mount("/", routes![meet_handler])
        .launch();
}
