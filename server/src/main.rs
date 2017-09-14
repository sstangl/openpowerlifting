#![feature(plugin)]
#![plugin(rocket_codegen)]

#![recursion_limit="256"] // For Diesel.
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

use diesel::prelude::*;

extern crate rocket;
use rocket::http::Cookies;

use std::error::Error;
use std::path::{Path, PathBuf};

pub mod schema;
use schema::Entry;
use schema::Meet;


fn get_db_connection() -> ConnectionResult<SqliteConnection> {
    SqliteConnection::establish("../build/openpowerlifting.sqlite3")
}


#[get("/")]
fn index(cookies: Cookies) -> &'static str {
    "Hello, world!"
}


// TODO: Don't use Box<Error> -- use a custom error type?
#[get("/meet/<meetpath..>")]
fn meet_handler(meetpath: PathBuf) -> Result<String, Box<Error>> {
    let db = try!(get_db_connection());
    let meetpath_str = try!(meetpath.to_str().ok_or(
        std::io::Error::new(std::io::ErrorKind::Other, "Malformed string.")));

    let results = schema::meets::table.filter(schema::meets::MeetPath.eq(meetpath_str))
                  .limit(1)
                  .load::<Meet>(&db)
                  .expect("Error loading meet.");

    if results.len() == 0 {
        return Ok(String::from("Meet not found."));
    }

    let meetid: i32 = results[0].id.unwrap();

    let entries = schema::entries::table.filter(schema::entries::MeetID.eq(meetid))
                  .load::<Entry>(&db)
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
        .mount("/", routes![index])
        .mount("/", routes![meet_handler])
        .launch();
}
