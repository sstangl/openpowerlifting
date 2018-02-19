#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate accept_language;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::{Outcome, State};
use rocket::http::{Cookies, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::response::NamedFile;
use rocket_contrib::Template;

use std::env;
use std::path::{Path, PathBuf};
use std::process;

extern crate server;
use server::langpack::Language;
use server::opldb;
use server::pages;

/// Request guard for reading the "Accept-Language" HTTP header.
struct AcceptLanguage(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AcceptLanguage, ()> {
        let keys: Vec<_> = request.headers().get("Accept-Language").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptLanguage(None)),
            1 => Outcome::Success(AcceptLanguage(Some(keys[0].to_string()))),
            _ => return Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

fn select_display_language(languages: AcceptLanguage, cookies: Cookies) -> Language {
    let default = Language::en_US;

    // The user may explicitly override the language choice by using
    // a cookie named "lang".
    if let Some(cookie) = cookies.get("lang") {
        let value: &str = cookie.value();
        if let Ok(lang) = value.parse::<Language>() {
            return lang;
        }
    }

    // If a language was not explicitly selected, the Accept-Language HTTP
    // header is consulted, defaulting to English.
    match languages.0 {
        Some(s) => {
            // TODO: This vector should be static and in langpack.
            let known_languages = vec!["en-US", "ru"];
            let valid_languages = accept_language::intersection(&s, known_languages);

            if valid_languages.len() == 0 {
                default
            } else {
                valid_languages[0].parse::<Language>().unwrap_or(default)
            }
        }
        None => default,
    }
}

#[get("/static/<file..>")]
fn statics(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/u/<username>")]
fn lifter(
    username: String,
    opldb: State<opldb::OplDb>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let lang = select_display_language(languages, cookies);
    println!("{:?}", lang);

    let lifter_id = match opldb.get_lifter_id(&username) {
        None => return None,
        Some(id) => id,
    };

    let context = pages::lifter::Context::new(&opldb, lang, lifter_id);
    Some(Template::render("lifter", &context))
}

#[get("/m/<meetpath..>")]
fn meet(meetpath: PathBuf, opldb: State<opldb::OplDb>) -> Option<Template> {
    let meetpath_str: &str = match meetpath.to_str() {
        None => return None,
        Some(s) => s,
    };
    let meet_id = match opldb.get_meet_id(meetpath_str) {
        None => return None,
        Some(id) => id,
    };

    let context = pages::meet::Context::new(&opldb, meet_id);
    Some(Template::render("meet", &context))
}

fn rocket(opldb: opldb::OplDb) -> rocket::Rocket {
    // Initialize the server.
    rocket::ignite()
        .manage(opldb)
        .mount("/", routes![lifter, meet, statics])
        .attach(Template::fairing())
}

fn get_envvar_or_exit(key: &str) -> String {
    env::var(key)
        .map_err(|_| {
            eprintln!("Environment variable '{}' not set.", key);
            process::exit(1);
        })
        .unwrap()
}

fn main() {
    // Populate std::env with the contents of any .env file.
    dotenv::from_filename("server.env").ok();

    let lifters_csv = get_envvar_or_exit("LIFTERS_CSV");
    let meets_csv = get_envvar_or_exit("MEETS_CSV");
    let entries_csv = get_envvar_or_exit("ENTRIES_CSV");

    let opldb = match opldb::OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error loading OplDb: {}", e);
            process::exit(1);
        }
    };

    println!("OplDb loaded in {}MB.", opldb.size_bytes() / 1024 / 1024);

    // Run the server loop.
    rocket(opldb).launch();
}
