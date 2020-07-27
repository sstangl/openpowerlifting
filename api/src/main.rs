//! The OpenPowerlifting data API server.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket::request::Request;
use rocket::response::Responder;
use rocket::State;

use rocket_contrib::json::Json;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

use langpack::{LangInfo, Locale};

mod beta;

/// Return type for pre-rendered Json strings.
#[derive(Debug)]
pub struct JsonString(pub String);

impl Responder<'static> for JsonString {
    fn respond_to(self, req: &Request) -> rocket::response::Result<'static> {
        rocket::response::content::Json(self.0).respond_to(req)
    }
}

#[catch(404)]
fn not_found() -> &'static str {
    "404"
}

#[catch(500)]
fn internal_error() -> &'static str {
    "500"
}

// Tests want to load the data only once.
#[cfg(not(test))]
type ManagedOplDb = opldb::OplDb;
#[cfg(test)]
type ManagedOplDb = &'static opldb::OplDb;

#[post("/rankings", data = "<options>")]
fn beta_rankings_default(
    options: Json<beta::RankingsOptions>,
    opldb: State<ManagedOplDb>,
    langinfo: State<LangInfo>,
) -> Option<JsonString> {
    let query = opldb::query::direct::RankingsQuery::default();
    let locale = Locale::new(&langinfo, options.language, options.units);
    let res = beta::RankingsReturn::from(&opldb, &locale, &query, &options);
    Some(JsonString(serde_json::to_string(&res).ok()?))
}

#[post("/rankings/<selections..>", data = "<options>")]
fn beta_rankings(
    selections: Option<PathBuf>,
    options: Json<beta::RankingsOptions>,
    opldb: State<ManagedOplDb>,
    langinfo: State<LangInfo>,
) -> Option<JsonString> {
    // The specific kind of rankings are encoded in the URL.
    let defaults = opldb::query::direct::RankingsQuery::default();
    let query = match selections {
        None => defaults,
        Some(path) => opldb::query::direct::RankingsQuery::from_url_path(&path, &defaults).ok()?,
    };

    let locale = Locale::new(&langinfo, options.language, options.units);
    let res = beta::RankingsReturn::from(&opldb, &locale, &query, &options);
    Some(JsonString(serde_json::to_string(&res).ok()?))
}

/// Connects the server endpoints together.
fn rocket(opldb: ManagedOplDb, langinfo: LangInfo) -> rocket::Rocket {
    rocket::ignite()
        .manage(opldb)
        .manage(langinfo)
        .mount("/beta/", routes![beta_rankings_default, beta_rankings])
        .register(catchers![not_found, internal_error])
        .attach(rocket::fairing::AdHoc::on_response(
            "Delete Server Header",
            |_request, response| {
                response.remove_header("Server");
            },
        ))
}

/// Loads in the database and starts the server.
fn main() -> Result<(), Box<dyn Error>> {
    // Accept an optional "--set-cwd" argument to manually specify the
    // current working directory. This allows the binary and the data
    // to be separated on a production server.
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "--set-cwd" {
        let fileroot = Path::new(&args[2]);
        env::set_current_dir(&fileroot).expect("Invalid --set-cwd argument");
    }

    // Populate std::env with the contents of any .env file.
    dotenv::from_filename("server.env").expect("Couldn't find server.env");

    // Load the OplDb.
    let start = std::time::Instant::now();
    let lifters_csv = env::var("LIFTERS_CSV").expect("LIFTERS_CSV not set");
    let meets_csv = env::var("MEETS_CSV").expect("MEETS_CSV not set");
    let entries_csv = env::var("ENTRIES_CSV").expect("ENTRIES_CSV not set");
    let opldb = opldb::OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv)?;
    println!(
        "DB loaded in {}MB and {:#?}.",
        opldb.size_bytes() / 1024 / 1024,
        start.elapsed()
    );

    let langinfo = LangInfo::new();

    #[cfg(not(test))]
    rocket(opldb, langinfo).launch();
    Ok(())
}
