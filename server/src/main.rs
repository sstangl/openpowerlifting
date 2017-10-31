#![feature(plugin)]
#![plugin(rocket_codegen)]

#![recursion_limit="256"] // For Diesel.
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;


extern crate r2d2_diesel;
extern crate r2d2;

use diesel::prelude::*;

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use rocket::response::NamedFile;

use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

mod schema;
use schema::Entry;
use schema::Meet;
use schema::DbConn;

mod queries;


#[get("/static/<file..>")]
fn static_handler(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


#[get("/index.html")]
fn index_html() -> Option<NamedFile> {
    NamedFile::open("htmltmp/index.html").ok()
}

#[get("/contact.html")]
fn contact_html() -> Option<NamedFile> {
    NamedFile::open("htmltmp/contact.html").ok()
}

#[get("/data.html")]
fn data_html() -> Option<NamedFile> {
    NamedFile::open("htmltmp/data.html").ok()
}

#[get("/lifters.html")]
fn lifters_html() -> Option<NamedFile> {
    NamedFile::open("htmltmp/lifters.html").ok()
}

#[get("/meet.html")]
fn meet_html() -> Option<NamedFile> {
    NamedFile::open("htmltmp/meet.html").ok()
}

#[get("/meetlist.html")]
fn meetlist_html() -> Option<NamedFile> {
    NamedFile::open("htmltmp/meetlist.html").ok()
}


#[get("/")]
fn index() -> Option<NamedFile> {
    index_html()
}


// TODO: Don't use Box<Error> -- use a custom error type?
#[get("/meet/<meetpath..>")]
fn meet_handler(meetpath: PathBuf, conn: DbConn) -> Result<String, Box<Error>> {
    let meetpath_str = try!(meetpath.to_str().ok_or(
        std::io::Error::new(std::io::ErrorKind::Other, "Malformed string.")));

    let meet_option = queries::get_meet_by_meetpath(meetpath_str, &conn);
    if meet_option.is_none() {
        return Ok(String::from("Meet not found."));
    }
    let meet = meet_option.unwrap();

    let entries_option = queries::get_entries_by_meetid(meet.id, &conn);
    if entries_option.is_none() {
        return Ok(String::from("Error loading entries."));
    }
    let entries = entries_option.unwrap();

    let mut display = String::new();

    for entry in entries {
        display.push_str(format!("{} - {}\n", entry.name, entry.sex).as_str());
    }

    Ok(display)
}


#[get("/lifter/<name>")]
fn lifter_handler(name: String, conn: DbConn) -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("lifter", &context)
}


fn main() {
    rocket::ignite()
        .manage(schema::init_pool())
        .mount("/", routes![index])
        .mount("/", routes![static_handler])
        .mount("/", routes![lifter_handler])
        .mount("/", routes![meet_handler])

        // Old HTML handlers.
        .mount("/", routes![index_html, contact_html, data_html,
                            lifters_html, meet_html, meetlist_html])

        .attach(Template::fairing())
        .launch();
}
